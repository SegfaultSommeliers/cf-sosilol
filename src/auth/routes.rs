// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::{
        AppError,
        AppState,
    },
    auth::{
        OAuthSession,
        Session,
        oauth,
        service,
    },
    models::Paste,
    pages,
};
use axum::{
    extract::{
        Query,
        State,
    },
    response::{
        IntoResponse,
        Redirect,
        Response,
    },
};
use axum_extra::extract::{
    PrivateCookieJar,
    cookie::{
        Cookie,
        SameSite,
    },
};
use reqwest::Url;
use serde::Deserialize;
use thiserror::Error;
use time::{
    Duration,
    OffsetDateTime,
};

#[derive(Debug, Error)]
enum Error {
    #[error("missing state")]
    MissingState,

    #[error("state mismatch or expired")]
    InvalidState,

    #[error("generating authorization URL")]
    AuthorizationUrl(#[source] oauth::Error),

    #[error("serializing session")]
    SerializingSession(#[source] serde_json::Error),

    #[error("invalid session user ID")]
    InvalidUserId(#[source] std::num::ParseIntError),

    #[error("authentication service")]
    Service(#[source] service::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::MissingState => Self::bad_request(error.to_string()),
            Error::InvalidState => Self::unauthorized(error.to_string()),
            error => Self::internal(error),
        }
    }
}

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect: Option<String>,
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

fn safe_redirect(value: &str) -> bool {
    value.starts_with('/')
        && !value.starts_with("//")
        && !value.contains('\\')
        && !value.chars().any(char::is_control)
}

fn cookie(
    name: &'static str,
    value: String,
    state: &AppState,
    seconds: i64,
) -> Cookie<'static> {
    Cookie::build(Cookie::new(name, value))
        .http_only(true)
        .secure(state.redirect_url.starts_with("https://"))
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(seconds))
        .path("/")
        .build()
}

pub fn session(jar: &PrivateCookieJar) -> Option<Session> {
    let session: Session = serde_json::from_str(jar.get("session")?.value()).ok()?;
    let now = OffsetDateTime::now_utc().unix_timestamp() as u64;
    (session.exp > now).then_some(session)
}

pub async fn request(
    State(state): State<AppState>,
    Query(query): Query<LoginQuery>,
    jar: PrivateCookieJar,
) -> Result<Response, AppError> {
    let mut oauth = state
        .auth_service(&state.auth_storage())
        .generate_auth_cookie()
        .map_err(Error::Service)?;
    oauth.redirect = query
        .redirect
        .filter(|v| safe_redirect(v))
        .unwrap_or_else(|| "/profile".into());
    let url = state
        .auth_provider
        .get_auth_url(
            &oauth.code_challenge,
            &oauth.state,
            &state.redirect_url,
            &["read:user"],
        )
        .map_err(Error::AuthorizationUrl)?;
    let value = serde_json::to_string(&oauth).map_err(Error::SerializingSession)?;
    Ok((
        jar.add(cookie("auth_session", value, &state, 600)),
        Redirect::to(url.as_str()),
    )
        .into_response())
}

#[worker::send]
pub async fn callback(
    State(state): State<AppState>,
    Query(query): Query<CallbackQuery>,
    jar: PrivateCookieJar,
) -> Result<Response, AppError> {
    let oauth: OAuthSession = jar
        .get("auth_session")
        .and_then(|v| serde_json::from_str(v.value()).ok())
        .ok_or(Error::MissingState)?;
    let now = OffsetDateTime::now_utc().unix_timestamp() as u64;
    if oauth.exp <= now || query.state.is_empty() || query.state != oauth.state {
        return Err(Error::InvalidState.into());
    }
    let (token, user) = state
        .auth_service(&state.auth_storage())
        .authorize(&state.auth_provider, &oauth.code_verifier, &query.code)
        .await
        .map_err(Error::Service)?;
    let avatar_url = Url::parse(&user.avatar_url)
        .ok()
        .filter(|url| {
            url.scheme() == "https"
                && matches!(
                    url.host_str(),
                    Some("avatars.githubusercontent.com" | "github.com")
                )
        })
        .map(|_| user.avatar_url)
        .unwrap_or_default();
    let session = Session {
        user_id: user.id.to_string(),
        github_access_token: token,
        login: user.login,
        avatar_url,
        exp: now + 86400 * 30,
    };
    let value = serde_json::to_string(&session).map_err(Error::SerializingSession)?;
    let redirect = if safe_redirect(&oauth.redirect) {
        oauth.redirect
    } else {
        "/profile".into()
    };
    let jar = jar
        .remove(cookie("auth_session", String::new(), &state, 0))
        .add(cookie("session", value, &state, 86400 * 30));
    Ok((jar, Redirect::to(&redirect)).into_response())
}

pub async fn logout(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Response {
    (
        jar.remove(cookie("session", String::new(), &state, 0)),
        Redirect::to("/"),
    )
        .into_response()
}

#[worker::send]
pub async fn profile(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<Response, AppError> {
    let Some(session) = session(&jar) else {
        return Ok(Redirect::to("/auth/request").into_response());
    };
    let user_id: i64 = session.user_id.parse().map_err(Error::InvalidUserId)?;
    let ids = state
        .auth_service(&state.auth_storage())
        .profile_pastes(user_id)
        .await
        .map_err(Error::Service)?;
    let pastes: Vec<_> = ids.iter().map(|id| Paste { id }).collect();
    Ok(pages::profile(&session.login, &session.avatar_url, &pastes).into_response())
}

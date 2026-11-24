use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub exp: u64,
    pub github_access_token: String,
    pub login: String,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthSession {
    pub state: String,
    pub code_verifier: String,
    pub code_challenge: String,
    pub redirect: String,
    pub exp: u64,
}

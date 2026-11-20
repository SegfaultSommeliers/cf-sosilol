use crate::auth::oauth::Provider;

impl Provider {
    pub fn github(
        client_id: String,
        client_secret: String,
    ) -> Provider {
        Self::new(
            "https://github.com/login/oauth/authorize",
            "https://github.com/login/oauth/access_token",
            client_id,
            client_secret,
        )
    }
}

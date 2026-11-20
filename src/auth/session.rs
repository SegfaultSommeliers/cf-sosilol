use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub exp: u64,
    pub github_access_token: String,
}

use crate::models::Paste;

pub struct Profile<'a> {
    pub id: i64,
    pub login: &'a str,
    pub avatar_url: &'a str,
    pub pastes: &'a [Paste<'a>],
}

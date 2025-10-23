use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct InputUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) company: String,
    pub(crate) exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub(crate) email: String,
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) admin: bool,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) admin: bool,
    pub(crate) email: String,
    pub(crate) username: String,
}
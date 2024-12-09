use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
}
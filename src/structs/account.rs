use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccountRequestByIDRequest {
    pub(crate) id: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccountRequestByEmail {
    pub(crate) email: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) company: String,
    pub(crate) exp: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Account {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) admin: bool,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub(crate) id: String,
    pub(crate) admin: bool,
    pub(crate) email: String,
    pub(crate) username: String,
}
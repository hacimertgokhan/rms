use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct SectionsByID {
    pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Sections {
    pub(crate) id: String,
    pub(crate) uuid: String,
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) full: bool,
    pub(crate) total: i32,
}
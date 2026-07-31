use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "material_status", rename_all = "lowercase")]
pub enum MaterialStatus {
    Draft,
    Published,
    Archived
}
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "material_type", rename_all = "lowercase")]
pub enum MaterialType {
    Lesson,
    Resource,
    Exercise,
    Quiz,
    Assignment,
}

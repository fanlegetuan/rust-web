use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
pub struct Course {
    pub teacher_id: i32,
    #[serde(default)]
    pub id: Option<i32>,
    pub name: String,
    #[serde(default)]
    pub time: Option<NaiveDateTime>,
}

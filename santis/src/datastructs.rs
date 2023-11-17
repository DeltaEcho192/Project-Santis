use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Item {
    pub item_id: Option<Uuid>,
    pub item_name: String,
    pub size: i64,
    pub weight: i64,
    pub value: i64, 
    pub packed: String,
    pub category: String,
    pub sub_category: String
}


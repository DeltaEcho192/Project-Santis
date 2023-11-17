use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum PackedDynamic {
    String(String),
    Int(i64)
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Item {
    pub item_id: Option<Uuid>,
    pub item_name: String,
    pub size: String,
    pub weight: i64,
    pub value: i64, 
    pub packed: PackedDynamic,
    pub category: String,
    pub sub_category: String
}


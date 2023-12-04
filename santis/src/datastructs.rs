use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Item {
    pub item_id: Option<Uuid>,
    pub item_name: String,
    pub size: String,
    pub weight: i64,
    pub value: i64, 
    pub packed: Option<i64>,
    pub category: String,
    pub sub_category: String,
    pub box_num: Option<i64>
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct ItemEdit {
    pub item_id: String,
    pub item_name: String,
    pub category: String,
    pub box_num: i64
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct ItemSave{
    pub item_name: String,
    pub category: String,
    pub box_num: i64
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Search {
    pub search: String,
    pub box_num: i64
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct BoxItem {
    pub box_id: i64,
    pub weight: i64
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct BoxEdit{
    pub weight: i64
}

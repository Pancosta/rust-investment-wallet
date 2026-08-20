use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
}

#[derive(Clone)]
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PurchaseHistory {
    #[serde(with = "time::serde::iso8601")]
    pub bought_at: OffsetDateTime,
    pub bought_for: f64,
    pub quantity_bought: f64,
    pub value_delta: f64,
}

#[derive(Serialize)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Vec<PurchaseHistory>,
}

#[derive(Clone)]
pub struct PurchaseRecord {
    pub user_id: i64,
    pub asset_id: i64,
    pub quantity: f64,
    pub bought_for: f64,
    pub bought_at: OffsetDateTime,
}

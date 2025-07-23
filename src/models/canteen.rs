use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::models::Store;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canteen {
    pub id: i64,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub image_url: String,
    pub stores: Vec<Store>,
}

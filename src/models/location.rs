use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::models::Store;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub stores: Vec<Store>,
}

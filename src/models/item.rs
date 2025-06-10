use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub price: BigDecimal,
    pub is_available: bool,
    pub description: String,
}

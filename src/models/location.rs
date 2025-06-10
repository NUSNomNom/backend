use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub is_open: bool,
    pub cuisine: String,
    pub description: String,
}

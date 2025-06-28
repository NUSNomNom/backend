use serde::{Deserialize, Serialize};

use crate::models::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub is_open: bool,
    pub cuisine: String,
    pub information: String,
    pub image_url: String,
    pub items: Vec<Item>,
}

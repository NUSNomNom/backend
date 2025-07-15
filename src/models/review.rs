use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub id: i64,
    pub nomer_id: i64,
    pub store_id: i64,
    pub score: i64,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}

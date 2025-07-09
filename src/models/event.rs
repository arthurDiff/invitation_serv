use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub budget: f64,
    pub timeframe: super::DateRange,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

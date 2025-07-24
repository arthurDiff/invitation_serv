use chrono::{DateTime, Utc};
use sqlx;
use uuid::Uuid;

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "member_role", rename_all = "lowercase")]
pub enum MemberRole {
    Owner,
    Facilitator,
    Attendee,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub user_id: String,
    pub email_id: Uuid,
    pub role: MemberRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

use serde::Deserialize;
use serde::Serialize;
use chrono::NaiveDate;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_super: bool,
    pub iva: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct Organisation {
    pub org_id: Uuid,
    pub name: String,
    pub picture: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessNotification {
    pub notification_id: i32,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    // option because a none means pending
    pub is_accepted: Option<bool>,
    pub description: String,
}

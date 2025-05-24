use serde::{Serialize, Deserialize};
 use crate::domain::organisation::OrgID;
use chrono::NaiveDate;
use sqlx::FromRow;
use crate::domain::user::UserID;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessNotification {
    pub notification_id: i32,
    pub org_id: OrgID,
    pub user_id: UserID,
    pub date: NaiveDate,
    // option because a none means pending
    pub is_accepted: Option<bool>,
    pub description: String,
}

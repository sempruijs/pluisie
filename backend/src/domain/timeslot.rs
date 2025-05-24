use chrono::NaiveDate;
use chrono::Utc;
use chrono::DateTime;
use crate::domain::user::UserID;
use crate::domain::organisation::OrgID;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct Timeslot {
    pub timeslot_id: Uuid,
    pub created: DateTime<Utc>,
    pub org_id: OrgID,
    pub user_id: UserID,
    pub date: NaiveDate,
    pub hour: Vec<u8>, 
    pub is_enrolled: bool,
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow, ToSchema, Hash)]
pub struct Day {
    #[schema(value_type = String)]
    pub date: NaiveDate,
    pub hours: Vec<Hour>
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow, ToSchema, Hash)]
pub struct Hour {
    pub hour: u8,
    pub people_amount: u8,
}



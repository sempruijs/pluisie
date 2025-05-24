use crate::domain::timeslot::Day;
use crate::repository::timeslot::*;
use rocket::async_trait;
use chrono::NaiveDate;
use crate::domain::organisation::OrgID;
use crate::domain::user::UserID;

#[async_trait]
pub trait TimeslotService: Send + Sync {
    async fn subscribe_to_hours(&self, date: NaiveDate, hours: Vec<u8>, is_enrolled: bool, user_id: UserID, org_id: OrgID) -> Result<(), sqlx::Error>;

    async fn get_days(&self, user_id: &UserID, org_id: &OrgID, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<Vec<Day>, sqlx::Error>;
}

pub struct TimeslotServiceImpl<T: TimeslotRepository> {
    timeslot_repository: T,
}

impl<R: TimeslotRepository> TimeslotServiceImpl<R> {
    pub fn new(timeslot_repository: R) -> Self {
        Self { timeslot_repository }
    }
}

#[async_trait]
impl<R: TimeslotRepository> TimeslotService for TimeslotServiceImpl<R> {
    async fn subscribe_to_hours(&self, date: NaiveDate, hours: Vec<u8>, is_enrolled: bool, user_id: UserID, org_id: OrgID) -> Result<(), sqlx::Error> {
        self.timeslot_repository.subscribe_to_hours(date, hours, is_enrolled, user_id, org_id).await
    }

    async fn get_days(&self, user_id: &UserID, org_id: &OrgID, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<Vec<Day>, sqlx::Error> {
        self.timeslot_repository.get_days(user_id, org_id, start_date, end_date).await
    }
}

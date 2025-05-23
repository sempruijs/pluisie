use crate::domain::Day;
use crate::repository::timeslot::*;
use rocket::async_trait;
use uuid::Uuid;
use chrono::NaiveDate;

#[async_trait]
pub trait TimeslotService: Send + Sync {
    async fn subscribe_to_hours(&self, date: NaiveDate, hours: Vec<u8>, is_enrolled: bool, user_id: Uuid, org_id: Uuid) -> Result<(), sqlx::Error>;

    async fn get_days(&self, user_id: Uuid, org_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<Day>, sqlx::Error>;
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
    async fn subscribe_to_hours(&self, date: NaiveDate, hours: Vec<u8>, is_enrolled: bool, user_id: Uuid, org_id: Uuid) -> Result<(), sqlx::Error> {
        self.timeslot_repository.subscribe_to_hours(date, hours, is_enrolled, user_id, org_id).await
    }

    async fn get_days(&self, user_id: Uuid, org_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<Day>, sqlx::Error> {
        self.timeslot_repository.get_days(user_id, org_id, start_date, end_date).await
    }
}

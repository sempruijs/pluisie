use crate::domain::Timeslot;
use crate::repository::timeslot::*;
use rocket::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TimeslotService: Send + Sync {

    async fn create(&self, timeslot: Timeslot) -> Result<(), sqlx::Error>;
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
    async fn create(&self, timeslot: Timeslot) -> Result<(), sqlx::Error> {
        self.timeslot_repository.create(times-lot).await
    }
}

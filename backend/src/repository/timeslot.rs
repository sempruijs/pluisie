use crate::domain::Timeslot;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;
use chrono::Utc;

#[async_trait]
pub trait TimeslotRepository: Send + Sync {

    async fn create(&self, timeslot: Timeslot) -> Result<(), sqlx::Error>;

}

#[derive(Debug, Clone)]
pub struct TimeslotRepositoryImpl {
    pool: PgPool,
}

impl TimeslotRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TimeslotRepository for TimeslotRepositoryImpl {
    async fn create(&self, timeslot: Timeslot) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO timeslot (org_id, user_id, datum, hour, is_enrolled)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            timeslot.org_id,
            timeslot.user_id,
            timeslot.datum,
            timeslot.hour,
            timeslot.is_enrolled
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
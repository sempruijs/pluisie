use crate::domain::Timeslot;
use std::collections::BTreeMap;
use crate::domain::Day;
use crate::domain::Hour;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use std::collections::HashMap;
use chrono::NaiveDate;

#[async_trait]
pub trait TimeslotRepository: Send + Sync {

    async fn create(&self, timeslot: Timeslot) -> Result<(), sqlx::Error>;

    async fn get_days(&self, user_id: Uuid, org_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<Day>, sqlx::Error>;

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
            INSERT INTO timeslots (timeslot_id, created, org_id, user_id, date, hour, is_enrolled)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            timeslot.timeslot_id,
            timeslot.created,
            timeslot.org_id,
            timeslot.user_id,
            timeslot.date,
            timeslot.hour,
            timeslot.is_enrolled
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_days(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Day>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT date, hour, COUNT(*) AS people_amount
            FROM timeslots
            WHERE user_id = $1
              AND org_id = $2
              AND date BETWEEN $3 AND $4
              AND is_enrolled = true
            GROUP BY date, hour
            ORDER BY date, hour
            "#,
            user_id,
            org_id,
            start_date,
            end_date,
        )
        .fetch_all(&self.pool)
        .await?;

        let day_map: BTreeMap<NaiveDate, Vec<Hour>> = rows
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, row| {
                let date = row.date;
                let hour = row.hour as u8;
                let people_amount = row.people_amount.unwrap_or(0) as u8;

                acc.entry(date)
                    .or_default()
                    .push(Hour { hour, people_amount });

                acc
            });

        let days = day_map
            .into_iter()
            .map(|(date, mut hours)| {
                hours.sort_by_key(|h| h.hour);
                Day { date, hours }
            })
            .collect();

        Ok(days)
    }
}

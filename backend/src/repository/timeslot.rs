use sqlx::QueryBuilder;
use crate::domain::organisation::OrgID;
use crate::domain::user::UserID;
use std::collections::BTreeMap;
use crate::domain::timeslot::Day;
use crate::domain::timeslot::Hour;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;
use chrono::NaiveDate;

#[async_trait]
pub trait TimeslotRepository: Send + Sync {
    async fn get_days(&self, user_id: &UserID, org_id: &OrgID, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<Vec<Day>, sqlx::Error>;

    async fn subscribe_to_hours(&self, date: &NaiveDate, hours: &Vec<u8>, is_enrolled: &bool, user_id: &UserID, org_id: &OrgID) -> Result<(), sqlx::Error>;
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
    async fn get_days(
        &self,
        user_id: &UserID,
        org_id: &OrgID,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
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
            user_id.0,
            org_id.0,
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

    async fn subscribe_to_hours(
        &self,
        date: &NaiveDate,
        hours: &Vec<u8>,
        is_enrolled: &bool,
        user_id: &UserID,
        org_id: &OrgID,
    ) -> Result<(), sqlx::Error> {
        let mut builder = QueryBuilder::new(
            "INSERT INTO timeslots (timeslot_id, org_id, user_id, date, hour, is_enrolled) ",
        );

        builder.push("VALUES ");

        let mut separated = builder.separated(", ");
        let hours = hours.into_iter().map(|hour| *hour as i16).collect::<Vec<i16>>();

        for hour in hours {
            separated.push_bind(Uuid::new_v4())
                .push_bind(org_id)
                .push_bind(user_id)
                .push_bind(date)
                .push_bind(hour as i16)
                .push_bind(is_enrolled);
        }

        builder.build().execute(&self.pool).await?;

        Ok(())
    }

}

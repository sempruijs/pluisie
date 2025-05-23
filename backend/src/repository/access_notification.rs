use crate::domain::AccessNotification;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;
use chrono::Utc;

#[async_trait]
pub trait AccessNotificationRepository: Send + Sync {
    async fn get_access_notification(&self, user_id: UserID) -> Result<Vec<AccessNotification>, sqlx::Error>;

    async fn create_access_notification(&self, org_id: OrgID, user_id: UserID, description: String) -> Result<bool, sqlx::Error>;
}

#[derive(Debug, Clone)]
pub struct AccessNotificationRepositoryImpl {
    pool: PgPool,
}

impl AccessNotificationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccessNotificationRepository for AccessNotificationRepositoryImpl {
    async fn get_access_notification(&self, user_id: UserID) -> Result<Vec<AccessNotification>, sqlx::Error> {
        let access_notification = sqlx::query_as::<_,
        AccessNotification>(
            r#" 
            SELECT DISTINCT ON (org_id, user_id) * FROM access_notifications
            WHERE user_id = $1
            ORDER BY org_id, user_id, date DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(access_notification)
    }

    async fn create_access_notification(
        &self,
        org_id: OrgID,
        user_id: UserID,
        description: String,
    ) -> Result<bool, sqlx::Error> {
        let date = Utc::now().date_naive();

        let result = sqlx::query!(
            r#"
            INSERT INTO access_notifications (org_id, user_id, date, is_accepted, description)
            VALUES ($1, $2, $3, NULL, $4)
            "#,
            org_id,
            user_id,
            date,
            description
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

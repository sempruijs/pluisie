use crate::domain::AccessNotification;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait AccessNotificationRepository: Send + Sync {

    async fn get_access_notification_by_user_id(&self, user_id: Uuid, ) -> Result<Vec<AccessNotification>, sqlx::Error>;

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
    
    async fn get_access_notification_by_user_id(&self, user_id: Uuid) -> Result<Vec<AccessNotification>, sqlx::Error> {
        let access_notification = sqlx::query_as!::<_,
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
}

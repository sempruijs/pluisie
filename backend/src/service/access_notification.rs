use crate::domain::AccessNotification;
use crate::repository::access_notification::*;
use rocket::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AccessNotificationService: Send + Sync {
    async fn get_access_notification(&self, user_id: Uuid) -> Result<Vec<AccessNotification>, sqlx::Error>;
    
    async fn create_access_notification(&self, org_id: Uuid, user_id: Uuid, description: String) -> Result<bool, sqlx::Error>;
}

pub struct AccessNotificationServiceImpl<T: AccessNotificationRepository> {
    access_notification_repository: T,
}

impl<R: AccessNotificationRepository> AccessNotificationServiceImpl<R> {
    pub fn new(access_notification_repository: R) -> Self {
        Self { access_notification_repository }
    }
}

// Implement UserService trait for UserServiceImpl.
#[async_trait]
impl<R: AccessNotificationRepository> AccessNotificationService for AccessNotificationServiceImpl<R> {
    async fn get_access_notification(&self, user_id: Uuid) -> Result<Vec<AccessNotification>, sqlx::Error> {
        self.access_notification_repository.get_access_notification(user_id).await
    }
    async fn create_access_notification(&self, org_id: Uuid, user_id: Uuid, description: String) -> Result<bool, sqlx::Error>{
        self.access_notification_repository.create_access_notification(org_id, user_id, description).await
    }
}


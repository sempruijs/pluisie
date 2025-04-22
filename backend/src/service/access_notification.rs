use crate::domain::AccessNotification;
use crate::repository::access_notification::*;
use rocket::async_trait;
use uuid::Uuid;

// Here you add your business logic here.
#[async_trait]
pub trait AccessNotificationService: Send + Sync {
    async fn get_access_notification_by_user_id(&self, user_id: Uuid) -> Result<Vec<AccessNotification>, sqlx::Error>;
}

pub struct AccessNotificationServiceImpl<T: AccessNotificationRepository> {
    access_notification_repository: T,
}

impl<R: AccessNotificationRepository> AccessNotificationServiceImpl<R> {
    // create a new function for UserServiceImpl.
    pub fn new(access_notification_repository: R) -> Self {
        Self { access_notification_repository }
    }
}

// Implement UserService trait for UserServiceImpl.
#[async_trait]
impl<R: AccessNotificationRepository> AccessNotificationService for AccessNotificationServiceImpl<R> {

    async fn get_access_notification_by_user_id(&self, user_id: Uuid) -> Result<Vec<AccessNotification>, sqlx::Error> {
        self.access_notification_repository.get_access_notification_by_user_id(user_id).await
    }

}


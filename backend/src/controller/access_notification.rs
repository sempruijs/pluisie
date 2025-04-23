use crate::service::access_notification::AccessNotificationService;
use rocket::get;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use crate::User;

//api::get_access_notification_by_user_id
#[derive(Debug, Serialize, Deserialize, ToSchema)]
 struct AccessNotificationResponse {
    pub org_id: String,
    pub user_id: String,
    pub date: String,
    pub is_accepted: Option<bool>,
    pub description: String,
 }

#[utoipa::path(
    get,
    path = "/access-notifications",
    responses(
        (status = 200, description = "Recieved notification list successfully", body = [AccessNotificationResponse]),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    ),
    description = "Get access notifications from user id",
    operation_id = "getAccessNotifications",
    tag = "Access Notification"
)]
#[get("/")]
async fn get_access_notification(
    service: &State<Arc<dyn AccessNotificationService>>,
    user: User
) -> Result<Json<Vec<AccessNotificationResponse>>, status::Custom<String>> {
    match service.get_access_notification(user.user_id).await {
        Ok(access_notifications) => {
            let response: Vec<AccessNotificationResponse> = access_notifications
                .into_iter()
                .map(|notification| AccessNotificationResponse {
                    org_id: notification.org_id.to_string(),
                    user_id: notification.user_id.to_string(),
                    date: notification.date.to_string(),
                    is_accepted: notification.is_accepted,
                    description: notification.description,
                })
                .collect();
            Ok(Json(response))
        }
        Err(e) => Err(status::Custom(
            rocket::http::Status::InternalServerError,
            format!("Database error: {}", e),
        )),
    }
}

// Combine all the access_notifications routes.
pub fn access_notification_routes() -> Vec<rocket::Route> {
    routes![
        get_access_notification
    ]
}

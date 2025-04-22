use crate::domain::AccessNotification;
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
use uuid::Uuid;

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
    path = "/access-notifications/{user_id}",
    params(
        ("user_id" = String, Path, description = "De ID van de gebruiker waarvoor notificaties worden opgehaald")
    ),
    responses(
        (status = 200, description = "Lijst van access notifications", body = [AccessNotificationResponse]),
        (status = 404, description = "Geen access notifications gevonden"),
        (status = 500, description = "Interne server fout"),
    ),
    description = "Get access notifications by user id",
    operation_id = "get_access_notification_by_user_id",
    tag = "AccessNotification"
)]
#[get("/access-notifications/<user_id>")]
async fn get_access_notification_by_user_id(
    user_id: String,
    service: &State<Arc<dyn AccessNotificationService>>,
) -> Result<Json<Vec<AccessNotificationResponse>>, status::Custom<String>> {
    let user_id = match Uuid::parse_str(&user_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(status::Custom(
                rocket::http::Status::BadRequest,
                "Invalid UUID format".to_string(),
            ));
        }
    };

    match service.get_access_notification_by_user_id(user_id).await{
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
        get_access_notification_by_user_id
    ]
}

use crate::service::access_notification::AccessNotificationService;
use chrono::NaiveDate;
use crate::domain::organisation::OrgID;
use crate::domain::user::UserID;
use rocket::post;
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
use rocket::http::Status;

//api::get_access_notification_by_user_id
#[derive(Debug, Serialize, Deserialize, ToSchema)]
 struct AccessNotificationResponse {
    pub org_id: OrgID,
    pub user_id: UserID,

    #[schema(value_type = String, format = "date", example = "2024-05-24")]
    pub date: NaiveDate,
    pub is_accepted: Option<bool>,
    pub description: String,
 }

#[utoipa::path(
    get,
    path = "/access-notification",
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
) -> Status {
    match service.get_access_notification(user.user_id).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
 struct CreateAccessNotificationRequest {
    pub org_id: OrgID,
    pub description: String,
 }

#[utoipa::path(
    post,
    path = "/access-notification",
    request_body = CreateAccessNotificationRequest, 
    responses(
        (status = 200, description = "Access notification created succesfully", body = bool),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    ),
    description = "Create access notification",
    operation_id = "CreateAccessNotifications",
    tag = "Access Notification"
)]
#[post("/", data = "<payload>")]
async fn create_access_notification(
    service: &State<Arc<dyn AccessNotificationService>>,
    user: User,
    payload: Json<CreateAccessNotificationRequest>,
) -> Status {
     match service
        .create_access_notification(payload.org_id.clone(), user.user_id, payload.description.clone())
        .await
    {
        Ok(true) => Status::Created,
        _ => Status::InternalServerError,
    }
}
// Combine all the access_notifications routes.
pub fn access_notification_routes() -> Vec<rocket::Route> {
    routes![
        get_access_notification,
        create_access_notification,
    ]
}

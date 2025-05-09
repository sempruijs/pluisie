use crate::service::timeslot::TimeslotService;
use uuid::Uuid;
use rocket::post;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use crate::User;

//api::post_timeslot
#[derive(Debug, Serialize, Deserialize, ToSchema)]
 struct CreateTimeslotRequest {
    pub org_id: String,
    pub date: String,
    pub hour: i16,
    pub is_enrolled: bool,
 }

#[utoipa::path(
    post,
    path = "/timeslot",
    request_body = CreateTimeslotRequest,
    responses(
        (status = 200, description = "Recieved timeslot list successfully", body = bool),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    ),
    description = "Create timeslot list",
    operation_id = "CreateTimeslot",
    tag = "Timeslot"
)]
#[post("/")]
async fn create_timeslot(
    service: &State<Arc<dyn TimeslotService>>,
    user: User,
    payload: Json<CreateTimeslotRequest>,
) -> Json<bool> {
let org_id = Uuid::parse_str(payload.org_id).expect("faild to parse org_id");
    let timeslot = Timeslot{
        org_id: payload.org_id.clone(),
        date: payload.date.clone(),
        hour: payload.hour.clone(),
        is_enrolled: payload.is_enrolled.clone(),
    }; 
    match service.create(timeslot).await{
        Ok(true) => Json(true),
        _ => Json(false),}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
 struct CreateAccessNotificationRequest {
    pub org_id: String,
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
) -> Json<bool> {
    let org_id = Uuid::parse_str(&payload.org_id).unwrap();

    match service.create_access_notification(org_id, user.user_id, payload.description.clone()).await {
        Ok(true) => Json(true),
        _ => Json(false),
    }
}

// Combine all the access_notifications routes.
pub fn access_notification_routes() -> Vec<rocket::Route> {
    routes![
        get_access_notification,
        create_access_notification,
    ]
}

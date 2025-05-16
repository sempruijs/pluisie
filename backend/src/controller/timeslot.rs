use crate::service::timeslot::TimeslotService;
use chrono::NaiveDate;
use crate::parser::*;
use chrono::Utc;
use crate::domain::Timeslot;
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

/// This endpoint is not secure because you can enter any type of org_id and it will add it regardless of the user role premissions
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
#[post("/", data = "<payload>")]
async fn create(
    service: &State<Arc<dyn TimeslotService>>,
    user: User,
    payload: Json<CreateTimeslotRequest>,
) -> Json<bool> {
    let org_id = Uuid::parse_str(&payload.org_id).expect("faild to parse org_id");
    let date: NaiveDate = parse_iso8601_date(&payload.date).expect("Failed to parse date");
    let timeslot = Timeslot {
        timeslot_id: Uuid::new_v4(),
        org_id,
        user_id: user.user_id,
        date,
        created: Utc::now(),
        hour: payload.hour.clone(),
        is_enrolled: payload.is_enrolled.clone(),
    }; 

    match service.create(timeslot).await {
        Ok(()) => Json(true),
        _ => Json(false),
    }
}

// Combine all the access_notifications routes.
pub fn timeslots_routes() -> Vec<rocket::Route> {
    routes![
        create
    ]
}

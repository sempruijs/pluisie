use crate::service::timeslot::TimeslotService;
use chrono::NaiveDate;
use crate::parser::*;
use chrono::Utc;
use crate::domain::User;
use crate::domain::Timeslot;
use crate::domain::Day;
use crate::domain::Hour;
use uuid::Uuid;
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

//api::subscribe_to_hours
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct SubscribeToHoursRequest {
    pub org_id: String,
    pub date: String,
    pub hours: Vec<u8>,
    pub is_enrolled: bool,
 }

/// This endpoint is not secure because you can enter any type of org_id and it will add it regardless of the user role premissions
#[utoipa::path(
    post,
    path = "/timeslot",
    request_body = SubscribeToHoursRequest,
    responses(
        (status = 200, description = "Subscribed to hours successfully", body = bool),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    ),
    description = "Subscribe to hours",
    operation_id = "SubcribeToHours",
    tag = "Timeslot"
)]
#[post("/", data = "<payload>")]
async fn subscirbe_to_hours(
    service: &State<Arc<dyn TimeslotService>>,
    user: User,
    payload: Json<SubscribeToHoursRequest>,
) -> Json<bool> {
    let org_id = Uuid::parse_str(&payload.org_id).expect("Faild to parse org_id");
    let date: NaiveDate = parse_iso8601_date(&payload.date).expect("Failed to parse date");
    let user_id = user.user_id;
    let hours = payload.hours.clone();
    let is_enrolled = payload.is_enrolled.clone();

    match service.subscribe_to_hours(date, hours, is_enrolled, user_id, org_id).await {
        Ok(()) => Json(true),
        _ => Json(false),
    }
}

//api::get_days
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetDaysRequest {
    pub org_id: String,
    pub start_date: String,
    pub end_date: String,
}
#[utoipa::path(
    get,
    path = "/timeslot",
    request_body = GetDaysRequest,
    responses(
        (status = 200, description = "Recieved timeslot list successfully", body = Vec<Day>),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    ),
    description = "Get timeslot list",
    operation_id = "GetTimeslot",
    tag = "Timeslot"
)]
#[get("/", data = "<payload>")]
async fn get_days(
    service: &State<Arc<dyn TimeslotService>>,
    user: User,
    payload: Json<GetDaysRequest>,
) -> Result<Json<Vec<Day>>, status::Custom<String>> {
    let org_id = Uuid::parse_str(&payload.org_id).expect("faild to parse org_id");
    let start_date: NaiveDate = parse_iso8601_date(&payload.start_date).expect("Failed to parse start date");
    let end_date: NaiveDate = parse_iso8601_date(&payload.end_date).expect("Failed to parse end date");

    match service.get_days(user.user_id, org_id, start_date, end_date).await {
        Ok(days) => Ok(Json(days)),
        Err(e) => Err(status::Custom(
            rocket::http::Status::InternalServerError,
            format!("Database error: {}", e),
        )),
    }
}

// Combine all the access_notifications routes.
pub fn timeslots_routes() -> Vec<rocket::Route> {
    routes![
        subscirbe_to_hours, get_days, 
    ]
}

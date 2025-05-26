use crate::service::timeslot::TimeslotService;
use crate::domain::organisation::OrgID;
use chrono::NaiveDate;
use crate::domain::user::User;
use crate::domain::timeslot::Day;
use rocket::post;
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
    pub org_id: OrgID,

    #[schema(value_type = String, format = "date", example = "2024-05-24")]
    pub date: NaiveDate,
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
) -> Result<Json<bool>, status::Custom<String>> {
    let p = payload;

    match service.subscribe_to_hours(&p.date, &p.hours, &p.is_enrolled, &user.user_id, &p.org_id).await {
        Ok(()) => Ok(Json(true)),
        Err(e) => Err(status::Custom(
            rocket::http::Status::InternalServerError,
            format!("Database error: {}", e),
        )),
    }
}

//api::get_days
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetDaysRequest {
    pub org_id: OrgID,

    #[schema(value_type = String, format = "date", example = "2024-05-24")]
    pub start_date: NaiveDate,

    #[schema(value_type = String, format = "date", example = "2024-07-24")]
    pub end_date: NaiveDate,
}

#[utoipa::path(
    post,
    path = "/timeslot/get",
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
#[post("/get", data = "<payload>")]
async fn get_days(
    service: &State<Arc<dyn TimeslotService>>,
    user: User,
    payload: Json<GetDaysRequest>,
) -> Result<Json<Vec<Day>>, status::Custom<String>> {
    let p = payload;

    match service.get_days(&user.user_id, &p.org_id, &p.start_date, &p.end_date).await {
        Ok(days) => Ok(Json(days)),
        Err(e) => Err(status::Custom(
            rocket::http::Status::InternalServerError,
            format!("Database error: {}", e),
        )),
    }
}

// Combine all the access_notifications routes.
pub fn timeslot_routes() -> Vec<rocket::Route> {
    routes![
        subscirbe_to_hours, get_days, 
    ]
}

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

//api::get_days
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetDaysrequest {
    pub user_id: String,
    pub org_id: String,
    pub start_date: String,
    pub end_date: String,
}
#[utoipa::path(
    get,
    path = "/timeslot",
    request_body = GetDaysrequest,
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
    payload: Json<GetDaysrequest>,
) -> Result<Json<Vec<Day>>, status::Custom<String>> {
    let user_id = Uuid::parse_str(&payload.user_id).expect("faild to parse user_id");
    let org_id = Uuid::parse_str(&payload.org_id).expect("faild to parse org_id");
    let start_date: NaiveDate = parse_iso8601_date(&payload.start_date).expect("Failed to parse start date");
    let end_date: NaiveDate = parse_iso8601_date(&payload.end_date).expect("Failed to parse end date");

    match service.get_days(user_id, org_id, start_date, end_date).await {
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
        create, get_days
    ]
}

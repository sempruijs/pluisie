use crate::domain::user::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::put;
use crate::service::user::UserService;
use rocket::get;
use rocket::post;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use chrono::NaiveDate;

/// Request body for creating a user.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub is_super: bool,
    pub iva: String,
    pub phone_number: String,
    #[schema(value_type = String, format = "date", example = "2024-05-24")]
    pub date_of_birth: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct UpdateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub iva: String,
    pub phone_number: String,
    #[schema(value_type = String, format = "date", example = "2024-05-24")]
    pub date_of_birth: NaiveDate,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Creates a user. The email should be unique.",
    operation_id = "createUser",
    tag = "Users"
)]
#[post("/", data = "<payload>")]
async fn create_user(
    payload: Json<CreateUserRequest>,
    user_service: &State<Arc<dyn UserService>>,
) -> Result<Status, Custom<String>>  {
    // Convert `CreateUserRequest` to `User`
    let user = User {
        user_id: UserID::new(),
        name: payload.name.clone(),
        email: payload.email.clone(),
        password: payload.password.clone(),
        // todo: this should be updated later.
        is_super: payload.is_super.clone(),
        iva: payload.iva.clone(),
        phone_number: payload.phone_number.clone(),
        date_of_birth: payload.date_of_birth.clone(),
    };

    // Call the `create` method and await its result
    match user_service.create(user).await {
        Ok(()) => Ok(Status::Created),
        Err(e) => {
            let msg = format!("Internal error: {e}");
            Err(status::Custom(Status::InternalServerError, msg))
        },
    }
}

#[utoipa::path(
    put,
    path = "/users",
    request_body = UpdateUserRequest,
    responses(
        (status = 201, description = "User updated successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Update a user, JWT is required",
    operation_id = "updateUser",
    tag = "Users"
)]
#[put("/", data = "<payload>")]
async fn update_user(
    payload: Json<UpdateUserRequest>,
    user_service: &State<Arc<dyn UserService>>,
    user: User,
) -> Result<Status, Custom<String>> {
    let updated_user = User {
        user_id: user.user_id,
        name: payload.name.clone(),
        email: payload.email.clone(),
        password: payload.password.clone(),
        // todo: this should be updated later.
        is_super: false,
        iva: payload.iva.clone(),
        phone_number: payload.phone_number.clone(),
        date_of_birth: payload.date_of_birth.clone(),
    };

  match user_service.update(updated_user).await {
        Ok(()) => Ok(Status::Ok),
        Err(e) => {
            let msg = format!("Internal error: {e}");
            Err(status::Custom(Status::InternalServerError, msg))
        },
    }
}

/// Response for recieving user information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetUserResponse {
    name: String,
    email: String,
    is_super: bool,
    iva: String,
    phone_number: String,
    #[schema(value_type = String, format = "date", example = "2024-05-24")]
    date_of_birth: NaiveDate,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 201, description = "User recieved successfully", body = GetUserResponse),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve user details.",
    operation_id = "getUser",
    tag = "Users",
    security(
        ("jwt_auth" = [])
    )
)]
#[get("/")]
async fn get_user(
    // user is recieved by decoding the JWT.
    // when a User is required as argument for an endpoiint, is automatically protected with JWT.
    user: User,
) -> Result<Json<GetUserResponse>, status::Custom<String>> {
    Ok(Json(GetUserResponse {
        email: user.email,
        name: user.name,
        is_super: user.is_super,
        iva: user.iva,
        phone_number: user.phone_number,
        date_of_birth: user.date_of_birth,
    }))

}

// Combine all the user routes.
pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user, get_user, update_user]
}

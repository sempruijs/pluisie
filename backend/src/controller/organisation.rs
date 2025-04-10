use crate::domain::Organisation;
use crate::domain::User;
use crate::service::user::UserService;
use crate::service::organisation::OrganisationService;
use rocket::post;
use rocket::delete;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;


/// Request body for creating a organisation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CreateOrganisationRequest {
    pub name: String,
    pub picture: String,
    pub description: String,
}

#[utoipa::path(
    post,
    path = "/organisation",
    request_body = CreateOrganisationRequest,
    responses(
        (status = 201, description = "Organisation created successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Creates a organisation. The name should be unique.",
    operation_id = "createOrganisation",
    tag = "Organisation"
)]
#[post("/", data = "<payload>")]
async fn create_organisation(
    payload: Json<CreateOrganisationRequest>,
    organisation_service: &State<Arc<dyn OrganisationService>>,
    user: User,
) -> Json<bool> {
    let organisation = Organisation {
        org_id: Uuid::new_v4(),
        name: payload.name.clone(),
        picture: Some(payload.picture.clone()),
        description: Some(payload.description.clone()),
    };

    if user.is_super {
         match organisation_service.create(organisation).await {
            Ok(()) => Json(true),
            Err(_) => Json(false),
        }
    } else {
        return Json(false)
    }
}

/// Request body for delete a organisation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DeleteOrganisationRequest {
    pub org_id: String,
}

#[utoipa::path(
    delete,
    path = "/organisation",
    request_body = DeleteOrganisationRequest,
    responses(
        (status = 201, description = "Organisation deleted successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "delete an organisation. The name should be unique.",
    operation_id = "DeleteOrganisation",
    tag = "Organisation"
)]
#[delete("/", data = "<payload>")]
async fn delete_organisation(
    payload: Json<DeleteOrganisationRequest>,
    organisation_service: &State<Arc<dyn OrganisationService>>,
    user: User,
) -> Json<bool> {

    if let Ok(org_id) = Uuid::parse_str(&payload.org_id) {
        if user.is_super {
         if let Ok(()) = organisation_service.delete(org_id).await {
            return Json(true);
           }
        } 
    }
    Json(false)
}


pub fn organisation_routes() -> Vec<rocket::Route> {
    routes![create_organisation, delete_organisation]
}

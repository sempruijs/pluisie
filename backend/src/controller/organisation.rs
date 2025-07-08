use crate::domain::organisation::Organisation;
use crate::domain::organisation::OrgID;
use crate::domain::user::User;
use crate::service::organisation::OrganisationService;
use rocket::post;
use rocket::delete;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use rocket::get; 
use rocket::http::Status;
use rocket::response::status::Custom;


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
) -> Status {
    let organisation = Organisation {
        org_id: OrgID::new(),
        name: payload.name.clone(),
        picture: Some(payload.picture.clone()),
        description: Some(payload.description.clone()),
    };

   if user.is_super {
        match organisation_service.create(organisation).await {
            Ok(()) => Status::Created,
            Err(_) => Status::InternalServerError,
        }
    } else {
        Status::Forbidden
    }
}
/// Request body for delete a organisation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DeleteOrganisationRequest {
    pub org_id: OrgID,
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
) -> Status {

       if user.is_super {
        match organisation_service.delete(payload.org_id.clone()).await {
            Ok(()) => Status::NoContent,
            Err(_) => Status::InternalServerError,
        }
         } else {
        Status::Forbidden
    }
}

/// Response for recieving organisation information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetOrganisationResponse {
    org_id: OrgID,
    name: String,
    picture: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetOrganisationRequest {
    pub org_id: OrgID,
}

#[utoipa::path(
    post,
    path = "/organisation/get",
    request_body = GetOrganisationRequest,
    responses(
        (status = 201, description = "organisation recieved successfully", body = GetOrganisationResponse),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve organisation details.",
    operation_id = "getOrganisation",
    tag = "Organisation",
    security(
        ("jwt_auth" = [])
    )
)]
#[post("/get", data = "<payload>")]
 async fn get_organisation(
    _user: User,
    payload: Json<GetOrganisationRequest>,
    organisation_service: &State<Arc<dyn OrganisationService>>,
) -> Status {
   if let Ok(Some(_)) = organisation_service.get_org_id(payload.org_id.clone()).await {
        return Status::Ok;
    }

 Status::NotFound
}
 

#[utoipa::path(
    get,
    path = "/organisation/all",
    responses(
        (status = 201, description = "Organisation recieved successfully", body = Vec<GetOrganisationResponse>),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Get all organisations",
    operation_id = "GetAllOrganisations",
    tag = "Organisation"
)]
#[get("/all")]
async fn get_all_organisation(
    organisation_service: &State<Arc<dyn OrganisationService>>,
    _user: User,
) -> Status {
    match organisation_service.get_all_org().await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
}

}
pub fn organisation_routes() -> Vec<rocket::Route> {
    routes![create_organisation, delete_organisation, get_organisation, get_all_organisation]
}

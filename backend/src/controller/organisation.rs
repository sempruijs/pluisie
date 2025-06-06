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
use rocket::response::status;
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
) -> Result<Status, Custom<String>> {
    let organisation = Organisation {
        org_id: OrgID::new(),
        name: payload.name.clone(),
        picture: Some(payload.picture.clone()),
        description: Some(payload.description.clone()),
    };

   if user.is_super {
        match organisation_service.create(organisation).await {
            Ok(()) => Ok(Status::Created),
            Err(e) => Err(status::Custom(
                rocket::http::Status::InternalServerError,
                format!("Database error: {}", e),
            )),
        }
    } else {
        Err(status::Custom(
            rocket::http::Status::Forbidden,
            "You are not allowed to create an organisation.".to_string(),
        ))
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
) -> Result<Status, Custom<String>> {

       if user.is_super {
        match organisation_service.delete(payload.org_id.clone()).await {
            Ok(()) => Ok(Status::NoContent), // 204 No Content bij succes
            Err(e) => Err(status::Custom(
                rocket::http::Status::InternalServerError,
                format!("Database error: {}", e),
            )),
        }
         } else {
        Err(status::Custom(
            rocket::http::Status::Forbidden,
            "You are not allowed to delete this organisation.".to_string(),
        ))
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
) -> Result<Json<GetOrganisationResponse>, Custom<String>> {
   if let Ok(Some(organisation)) = organisation_service.get_org_id(payload.org_id.clone()).await {
        return Ok(Json(GetOrganisationResponse {
            org_id: organisation.org_id,
            name: organisation.name,
            picture: organisation.picture.unwrap_or_default(),
            description: organisation.description.unwrap_or_default(),
        }));
    }

 Err(Custom(Status::NotFound, "Organisation not found".into()))
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
) -> Result<Json<Vec<GetOrganisationResponse>>, Custom<String>> {
    match organisation_service.get_all_org().await {
        Ok(organisations) => {
            let result = organisations.into_iter().map(|org| GetOrganisationResponse {
                org_id: org.org_id,
                name: org.name,
                picture: org.picture.unwrap(),
                description: org.description.unwrap(),
            }).collect::<Vec<GetOrganisationResponse>>();
            Ok(Json(result))
        },
        Err(_) =>Err(Custom(Status::InternalServerError, "Internal error".to_string()))
}

}
pub fn organisation_routes() -> Vec<rocket::Route> {
    routes![create_organisation, delete_organisation, get_organisation, get_all_organisation]
}

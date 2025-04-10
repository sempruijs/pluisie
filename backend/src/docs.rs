use crate::controller::authentication::*;
use crate::controller::user::*;
use crate::controller::organisation::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    create_user,
    get_user,
    login,
    update_user,
    create_organisation,
    delete_organisation
))]
pub struct ApiDoc;

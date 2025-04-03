use crate::controller::authentication::*;
use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    create_user,
    get_user,
    login,
    update_user,
))]
pub struct ApiDoc;

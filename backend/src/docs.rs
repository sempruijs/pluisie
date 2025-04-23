use crate::controller::authentication::*;
use crate::controller::user::*;
use crate::controller::organisation::*;
use crate::controller::access_notification::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    create_user,
    get_user,
    login,
    update_user,
    create_organisation,
    delete_organisation,
    get_organisation,
    get_all_organisation,
    get_access_notification
))]
pub struct ApiDoc;

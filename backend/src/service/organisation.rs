use crate::domain::Organisation;
use crate::repository::organisation::*;
use rocket::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait OrganisationService: Send + Sync {
    async fn create(&self, organisation: Organisation) -> Result<(), sqlx::Error>;

    async fn delete(&self, org_id: OrgID) -> Result<(), sqlx::Error>;

    async fn get_org_id(&self, org_id: OrgID) -> Result<Option<Organisation>, sqlx::Error>;

    async fn get_all_org(&self)-> Result<Vec<Organisation>, sqlx::Error>;
}

pub struct OrganisationServiceImpl<T: OrganisationRepository> {
    organisation_repository: T,
}

impl<R: OrganisationRepository> OrganisationServiceImpl<R> {
    pub fn new(organisation_repository: R) -> Self {
        Self { organisation_repository }
    }
}

#[async_trait]
impl<R: OrganisationRepository> OrganisationService for OrganisationServiceImpl<R> {
    async fn create(&self, organisation: Organisation) -> Result<(), sqlx::Error> {
        self.organisation_repository.create(organisation).await
    }

    async fn delete(&self, org_id: OrgID) -> Result<(), sqlx::Error> {
        self.organisation_repository.delete(org_id).await
    }

    async fn get_org_id(&self, org_id: OrgID) -> Result<Option<Organisation>, sqlx::Error> {
        self.organisation_repository.get_org_id(org_id).await
    }

    async fn get_all_org(&self)-> Result<Vec<Organisation>, sqlx::Error> {
        self.organisation_repository.get_all_org().await
    }

}

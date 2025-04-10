use rocket::async_trait;
use crate::domain::Organisation;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait OrganisationRepository: Send + Sync {
    async fn create(&self, organisation: Organisation) -> Result<(), sqlx::Error>;

    async fn delete(&self, org_id: Uuid) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Clone)]
pub struct OrganisationRepositoryImpl {
    pool: PgPool,
}

impl OrganisationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganisationRepository for OrganisationRepositoryImpl {
    async fn create(&self, organisation: Organisation) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO organisations (org_id, name, picture, description)
            VALUES ($1, $2, $3, $4)
            "#,
            organisation.org_id,
            organisation.name,
            organisation.picture,
            organisation.description,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, org_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM organisations
            WHERE org_id = $1
            "#,
            org_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

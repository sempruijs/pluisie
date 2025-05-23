use rocket::async_trait;
use crate::domain::Organisation;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait OrganisationRepository: Send + Sync {
    async fn create(&self, organisation: Organisation) -> Result<(), sqlx::Error>;

    async fn delete(&self, org_id: Uuid) -> Result<(), sqlx::Error>;

    async fn get_org_id(&self, org_id: Uuid)-> Result<Option<Organisation>, sqlx::Error>;

    async fn get_all_org(&self)-> Result<Vec<Organisation>, sqlx::Error>;

    // async fn get_by_user_id(user_id: UserID) -> Result<Vec<OrgID>, sqlx::Error>;
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

    async fn get_org_id(&self, org_id: Uuid) -> Result<Option<Organisation>, sqlx::Error> {
        let organisation = sqlx::query_as!(
            Organisation,
            r#"
            SELECT org_id, name, picture, description
            FROM organisations
            WHERE org_id = $1
            "#,
            org_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(organisation)
    }

    async fn get_all_org(&self)-> Result<Vec<Organisation>, sqlx::Error> {
        let organisations = sqlx::query_as::<_, Organisation>(
            "SELECT org_id, name, picture, description FROM organisations"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(organisations)

    }
}

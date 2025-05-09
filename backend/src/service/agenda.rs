use crate::domain::Agenda;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait AgendaRepository: Send + Sync {

    async fn create(&self, agenda: Agenda) -> Result<(), sqlx::Error>;

}

#[derive(Debug, Clone)]
pub struct AgendaRepositoryImpl {
    pool: PgPool,
}

impl AgendaRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgendaRepository for AgendaRepositoryImpl {
    async fn create(&self, agenda: Agenda) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO agenda (org_id, user_id, datum, hour, is_enrolled)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            agenda.org_id,
            agenda.user_id,
            agenda.datum,
            agenda.hour,
            agenda.is_enrolled
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
use crate::domain::User;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error>;

    async fn update(&self, updated_user: User) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (user_id, name, email, password, is_super, iva)
             VALUES ($1, $2, $3, $4, $5, $6)",
            user.user_id,
            user.name,
            user.email,
            user.password,
            user.is_super,
            user.iva
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, email, password, is_super, iva
             FROM users
             WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT user_id, name, email, password, is_super, iva
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, updated_user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET name = $1, email = $2, password = $3, is_super = $4, iva = $5 WHERE user_id = $6",
            updated_user.name,
            updated_user.email,
            updated_user.password,
            updated_user.is_super,
            updated_user.iva,
            updated_user.user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

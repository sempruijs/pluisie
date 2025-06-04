use crate::domain::user::User;
use crate::domain::user::UserID;
use rocket::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: UserID) -> Result<Option<User>, sqlx::Error>;

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
            "INSERT INTO users (user_id, name, email, password, is_super, iva, phone_number, date_of_birth)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            *user.user_id,
            user.name,
            user.email,
            user.password,
            user.is_super,
            user.iva,
            user.phone_number,
            user.date_of_birth
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn from_uuid(&self, user_id: UserID) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, email, password, is_super, iva, phone_number, date_of_birth
             FROM users
             WHERE user_id = $1",
            user_id.0
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT user_id, name, email, password, is_super, iva, phone_number, date_of_birth
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
            "UPDATE users SET name = $1, email = $2, password = $3, is_super = $4, iva = $5, phone_number = $6, date_of_birth = $7 WHERE user_id = $8",
            updated_user.name,
            updated_user.email,
            updated_user.password,
            updated_user.is_super,
            updated_user.iva,
            updated_user.phone_number,
            updated_user.date_of_birth,
            *updated_user.user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

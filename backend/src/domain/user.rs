use serde::{Serialize, Deserialize};
use std::ops::Deref;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: UserID,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_super: bool,
    pub iva: String,
    pub phone_number: String,
    pub date_of_birth: String,
}

#[derive(Debug, Eq, Clone, PartialEq, ToSchema, Serialize, Hash, Deserialize, FromRow, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserID(pub Uuid);

impl UserID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<&str> for UserID {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(value)?;
        Ok(UserID(uuid))
    }
}

impl Deref for UserID {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for UserID {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

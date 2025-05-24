use serde::{Serialize, Deserialize};
 use std::ops::Deref;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct Organisation {
    pub org_id: OrgID,
    pub name: String,
    pub picture: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Eq, Clone, PartialEq, ToSchema, Serialize, Hash, Deserialize, FromRow, sqlx::Type)]
#[sqlx(transparent)]
pub struct OrgID(pub Uuid);

impl OrgID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<&str> for OrgID {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(value)?;
        Ok(Self(uuid))
    }
}

impl Deref for OrgID {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for OrgID {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

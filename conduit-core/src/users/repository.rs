use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use conduit_domain::profiles::ProfileDto;
use conduit_domain::users::UserDto;
use mockall::automock;
use sqlx::postgres::PgRow;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::{FromRow, Row};

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn search_user_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> anyhow::Result<Option<UserEntity>>;

    async fn create_user(
        &self,
        email: &str,
        username: &str,
        hashed_password: &str,
    ) -> anyhow::Result<UserEntity>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;

    async fn update_user(
        &self,
        id: i64,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity>;
}

pub struct UserEntity {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            email: self.email,
            username: self.username,
            bio: self.bio,
            image: self.image,
            token,
        }
    }

    pub fn into_profile(self, following: bool) -> ProfileDto {
        ProfileDto {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: 1,
            bio: String::from("stub bio"),
            created_at: PrimitiveDateTime::from(SystemTime::now()),
            updated_at: PrimitiveDateTime::from(SystemTime::now()),
            username: String::from("stub username"),
            email: String::from("stub email"),
            password: String::from("stub password"),
            image: String::from("stub image"),
        }
    }
}

/// Implements a row/type mapping for sqlx to map our user entity directly into a scanned struct from a query.
impl<'a> FromRow<'a, PgRow> for UserEntity {
    fn from_row(row: &'a PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserEntity {
            id: row.get(0),
            created_at: row.get(1),
            updated_at: row.get(2),
            username: row.get(3),
            email: row.get(4),
            password: row.get(5),
            bio: row.get(6),
            image: row.get(7),
        })
    }
}

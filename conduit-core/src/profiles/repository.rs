use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::postgres::PgRow;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::{FromRow, Row};

pub type DynProfilesRepository = Arc<dyn ProfilesRepository + Send + Sync>;

pub struct UserFollowEntity {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub follower_id: i64,
    pub followee_id: i64,
}

#[automock]
#[async_trait]
pub trait ProfilesRepository {
    async fn get_user_followees(&self, user_id: i64) -> anyhow::Result<Vec<UserFollowEntity>>;
}

/// Implements a row/type mapping for sqlx to map our user follow entity directly into a scanned struct from a query.
impl<'a> FromRow<'a, PgRow> for UserFollowEntity {
    fn from_row(row: &'a PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserFollowEntity {
            id: row.get(0),
            created_at: row.get(1),
            follower_id: row.get(2),
            followee_id: row.get(3),
        })
    }
}

use crate::schema::*;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize)]
pub struct Score {
    pub id: i32,
    pub player_name: String,
    pub score: i64,
    pub created_at: DateTime<Utc>,
    pub player_uuid: Uuid,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "scores"]
pub struct NewScore<'x> {
    pub player_name: &'x str,
    pub score: i64,
    pub player_uuid: Uuid,
}

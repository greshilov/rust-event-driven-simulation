use crate::schema::*;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Score {
    pub id: i32,
    pub name: String,
    pub score: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "scores"]
pub struct NewScore<'x> {
    pub name: &'x str,
    pub score: i64,
}

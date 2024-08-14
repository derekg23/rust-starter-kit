use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::db::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

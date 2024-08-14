pub mod schema;
pub mod models; 

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;

#[derive(Clone)]
pub struct DbConn(Pool<ConnectionManager<PgConnection>>);

impl DbConn {
  pub fn get_pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
    &self.0
  }
}

pub fn establish_connection() -> DbConn {
  let db_username = std::env::var("DB_USERNAME").expect("DB_USERNAME must be set");
  let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
  let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
  let db_port = std::env::var("DB_PORT").expect("DB_PORT must be set");
  let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
  let manager = ConnectionManager::<PgConnection>::new(
    format!("postgres://{}:{}@{}:{}/{}", db_username, db_password, db_host, db_port, db_name)
  );
  let pool = Pool::builder().build(manager).expect("Failed to create pool.");
  DbConn(pool)
}

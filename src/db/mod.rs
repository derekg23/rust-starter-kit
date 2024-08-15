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
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  let pool = Pool::builder().build(manager).expect("Failed to create pool.");
  DbConn(pool)
}

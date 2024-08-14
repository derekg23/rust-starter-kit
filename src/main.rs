use actix_web::{web, App, HttpServer};
use actix_files as fs;
use routes::init;
use db::establish_connection;
use dotenv::dotenv;
use tera::Tera;

mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let pool = establish_connection();
  let tera = Tera::new("templates/**/*").unwrap();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone())) // Share the pool
      .app_data(web::Data::new(tera.clone())) // Share the Tera instance
      .configure(init)
      .service(fs::Files::new("/public", "./public").show_files_listing())
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

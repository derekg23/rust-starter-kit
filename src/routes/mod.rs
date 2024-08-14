use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};
mod users;

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
  let context = Context::new();
  // You can pass variables to your template here
  HttpResponse::Ok().content_type("text/html").body(tmpl.render("index.html", &context).unwrap())
}

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/")
      .route("", web::get().to(index)),
  );
  cfg.service(
    web::scope("/api/users")
      .service(web::resource("")
        .route(web::get().to(users::get_users))
        .route(web::post().to(users::create_user))
      )
      .service(web::resource("/{id}")
        .route(web::put().to(users::update_user))
        .route(web::delete().to(users::delete_user))
      )
  );
}

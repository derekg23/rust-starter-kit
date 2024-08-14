use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use crate::db::{DbConn, models::User, schema::users};

// Define the `NewUser` struct for deserialization
#[derive(Deserialize)]
pub struct NewUser {
  name: String,
  email: String,
}

// Define the `NewUserInsert` struct for Diesel insertable operations
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserInsert {
  pub name: String,
  pub email: String,
}

pub async fn get_users(db: web::Data<DbConn>) -> impl Responder {
  let pool = db.get_pool().clone();

  // Execute the query in a blocking context
  let users_list = web::block(move || {
    // Get a mutable connection from the pool
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    // Query the database for all users
    use crate::db::schema::users::dsl::*;
    users.load::<User>(&mut conn)
  })
  .await.unwrap();

  match users_list {
    Ok(users) => HttpResponse::Ok().json(users), // Serialize and return the users as JSON
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

pub async fn create_user(db: web::Data<DbConn>, user: web::Json<NewUser>) -> impl Responder {
  let pool = db.get_pool().clone();
  let new_user = user.into_inner();

  let result = web::block(move || {
    // Get a mutable connection from the pool
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    use crate::db::schema::users::dsl::*;

    let existing_user = users
      .filter(email.eq(&new_user.email))
      .first::<User>(&mut conn)
      .optional()?;

    if existing_user.is_some() {
      return Err(DieselError::NotFound);
    }

    diesel::insert_into(users)
      .values(&NewUserInsert {
        name: new_user.name,
        email: new_user.email,
      })
      .execute(&mut conn)
  }).await;

  match result {
    Ok(res) => match res {
      Ok(_) => HttpResponse::Created().finish(),
      Err(DieselError::NotFound) => HttpResponse::Conflict().body("User with this email already exists"),
      Err(_) => HttpResponse::InternalServerError().finish(),
    },
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

pub async fn update_user(
  db: web::Data<DbConn>,
  user_id: web::Path<i32>,
  user_data: web::Json<NewUser>,
) -> impl Responder {
  let pool = db.get_pool().clone();
  let user_id = user_id.into_inner();
  let update_data = user_data.into_inner();

  let result = web::block(move || {
    // Get a mutable connection from the pool
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    use crate::db::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(user_id)))
      .set((
        name.eq(update_data.name),
        email.eq(update_data.email),
      ))
      .execute(&mut conn)
  })
  .await;

  match result {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

pub async fn delete_user(
  db: web::Data<DbConn>,
  user_id: web::Path<i32>,
) -> impl Responder {
  let pool = db.get_pool().clone();
  let user_id = user_id.into_inner();

  let result = web::block(move || {
    // Get a mutable connection from the pool
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    use crate::db::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(user_id)))
      .execute(&mut conn)
  })
  .await;

  match result {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}
use crate::types::auth_types::SignupForm;
use actix_web::{HttpResponse, Responder, web, cookie::Cookie};
use bcrypt::{DEFAULT_COST, hash, verify};
use database::schema::user::dsl::*;
use database::{DbPool, models::user::User};
use diesel::prelude::*;
use serde_json::json;
use validator::Validate;
use uuid::Uuid;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use crate::utils::web_utils::get_jwt_secret;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user id
    exp: usize,   // expiration time
}

pub async fn signup_user(pool: web::Data<DbPool>, form: web::Json<SignupForm>) -> impl Responder {
    if let Err(e) = form.validate() {
        return HttpResponse::BadRequest().json(e);
    }

    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    match user.filter(email.eq(&form.email)).first::<User>(conn) {
        Ok(_) => {
            return HttpResponse::BadRequest().json(json!({ "message": "User already exists" }));
        }
        Err(diesel::result::Error::NotFound) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let hashed_password = hash(&form.password, DEFAULT_COST)
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .expect("Failed to hash password");

        let new_user = User {
            id: Uuid::new_v4(),
            email: form.email.clone(),
            password_hash: hashed_password,
            username: form.username.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(user)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .expect("Failed to insert user");

        let token = encode(
            &Header::default(),
            &Claims { sub: new_user.id.to_string(), exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize },
            &EncodingKey::from_secret(get_jwt_secret().as_bytes())
        ).unwrap();

        let cookie = Cookie::build("token", token)
        .path("/api/v1")
        .secure(true)
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({ "message": "Signup successful" }))

}

pub async fn login_user() -> impl Responder {
    log::info!("I'am user login handler!");
    HttpResponse::Ok().json(json!({ "message": "Login successful" }))
}

pub async fn get_user() -> impl Responder {
    log::info!("I'am user get handler!");
    HttpResponse::Ok().json(json!({ "message": "Get successful" }))
}

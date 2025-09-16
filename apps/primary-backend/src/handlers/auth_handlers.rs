use crate::types::auth_types::{SignupForm, LoginForm};
use actix_web::{HttpResponse, Responder, web, cookie::Cookie};
use bcrypt::{DEFAULT_COST, hash, verify};
use database::schema::users::dsl::*;
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

    match users.filter(email.eq(&form.email)).first::<User>(conn) {
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

        diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .expect("Failed to insert user");


        println!("new_user: {}", new_user.id);

        let token = encode(
            &Header::default(),
            &Claims { sub: new_user.id.to_string(), exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize },
            &EncodingKey::from_secret(get_jwt_secret().as_bytes())
        ).unwrap();

        println!("token: {}", token);

        let cookie = Cookie::build("token", token)
        .path("/api/v1")
        .secure(true)
        .http_only(true)
        .finish();

        println!("cookie: {:?}", cookie);

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({ "message": "Signup successful" }))
}

pub async fn login_user(
    pool: web::Data<DbPool>,
    form: web::Json<LoginForm>
) -> impl Responder {

    if let Err(e) = form.validate() {
        return HttpResponse::BadRequest().json(e);
    }

    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    match users.filter(email.eq(&form.email)).first::<User>(conn) {
        Ok(user) => {
            match verify(&form.password, &user.password_hash){
                Ok(true) => {
                    // Create JWT token
                    let claims = Claims {
                        sub: user.id.to_string(),
                        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                    };
                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(get_jwt_secret().as_bytes())
                    ).unwrap();

                    let cookie = Cookie::build("token", token)
                        .path("/api/v1")
                        .secure(true)
                        .http_only(true)
                        .finish();

                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(json!({ "message": "Login successful" }))
                },
                _ => {
                    HttpResponse::Unauthorized()
                        .json(json!({ "error": "Invalid credentials" }))
                }
                }
            },
            Err(diesel::result::Error::NotFound) => {
                HttpResponse::NotFound()
                    .json(json!({ "error": "User not found" }))
            },
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }


    pub async fn logout() -> impl Responder {
        HttpResponse::Ok()
            .cookie(
                Cookie::build("token", "")
                    .http_only(true)
                    .secure(true)
                    .path("/")
                    .expires(actix_web::cookie::time::OffsetDateTime::now_utc())  // Expire immediately
                    .finish()
            )
            .json(json!({ "message": "Logout successful" }))
    }

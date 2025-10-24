use crate::errors::internal_error;
use crate::types::inputs::InputUser;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use database::models::User;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::Uuid};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // user id
    exp: usize,  // expiration time
}

pub async fn signup_user(
    State(pool): State<PgPool>,
    Json(input_new_user): Json<InputUser>,
) -> impl IntoResponse {
    let new_user = User {
        id: Uuid::new_v4(),
        name: input_new_user.name,
        email: input_new_user.email,
        password_hash: input_new_user.password,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_local(),
    };

    let res = sqlx::query("INSERT INTO users (id, name, email, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(Uuid::new_v4())
        .bind(new_user.name)
        .bind(new_user.email)
        .bind(new_user.password_hash)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => {
            return (StatusCode::OK, "user creteated successfully");
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "user failed"),
    }
}

pub async fn signin_user() {
    println!("signin handler")
}

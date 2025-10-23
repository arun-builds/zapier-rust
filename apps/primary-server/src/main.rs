use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{ post},
};
use chrono::Utc;
use database::schema::users::dsl::*;
use database::{
    database::{DbPool, create_pool},
    models::user::User,
};
use diesel::prelude::*;
use types::inputs::InputUser;
use uuid::Uuid;

mod types;

#[tokio::main]
async fn main() {
    let db_pool: DbPool = create_pool().expect("Failed to create pool");

    let app = Router::new()
        .route("/user/create", post(create_user))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(pool): State<DbPool>,
    Json(input_new_user): Json<InputUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let new_user = User {
        id: Uuid::new_v4(),
        name: input_new_user.name,
        email: input_new_user.email,
        password_hash: input_new_user.password,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_local(),
    };
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

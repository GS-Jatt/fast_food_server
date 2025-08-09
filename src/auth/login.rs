use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{ResponseErrStr, User};

impl User {
    fn send(mut self) -> User {
        self.password = None;
        self
    }
}
#[derive(Deserialize)]
pub struct Payload {
    email: String,
    password: String,
}

pub async fn login(State(db): State<PgPool>, Json(payload): Json<Payload>) -> ResponseErrStr<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users where email = $1")
        .bind(&payload.email)
        .fetch_one(&db)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    if !payload.password.eq(user.password.as_ref().unwrap()) {
        return Err((StatusCode::UNAUTHORIZED, String::from("incorrect password")));
    }

    Ok((StatusCode::OK, Json(user.send())))
}
pub async fn login_with_id(State(db): State<PgPool>, Path(id): Path<Uuid>) -> ResponseErrStr<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users where id = $1")
        .bind(id)
        .fetch_one(&db)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    Ok((StatusCode::OK, Json(user.send())))
}

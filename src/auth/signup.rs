use axum::{Json, extract::State, http::StatusCode};
use sqlx::{PgPool, prelude::FromRow};

use crate::types::{ResponseErrStr, Singup, User};
#[derive(FromRow)]
struct Count {
    count: i64,
}

pub async fn signup(State(db): State<PgPool>, Json(data): Json<Singup>) -> ResponseErrStr<User> {
    let count: i64 = sqlx::query_as::<_, Count>(
        "
            SELECT COUNT(*) as count FROM users WHERE email = $1",
    )
    .bind(&data.email)
    .fetch_one(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .count;

    if count > 0 {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            String::from(" User already exists with this email"),
        ));
    }

    let res = sqlx::query_as::<_, User>(
        "INSERT INTO users ( name, email, password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(data.name)
    .bind(data.email)
    .bind(data.password)
    .fetch_one(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(User {
            id: res.id,
            name: res.name,
            email: res.email,
            password: None,
        }),
    ))
}

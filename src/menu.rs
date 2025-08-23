#![allow(non_snake_case)]
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct Menu {
    pub id: i32,
    pub name: String,
    pub unitPrice: i32,
    pub imageUrl: String,
    pub ingredients: Vec<String>,
    pub soldOut: bool,
}

pub async fn get_menu(State(db): State<PgPool>) -> (StatusCode, Result<Json<Vec<Menu>>, String>) {
    let menu = sqlx::query_as::<_, Menu>("SELECT  * FROM menus")
        .fetch_all(&db)
        .await;

    match menu {
        Ok(data) => (StatusCode::OK, Ok(Json(data))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string())),
    }
}

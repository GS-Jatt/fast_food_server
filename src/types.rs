use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub email: String,
    pub id: Uuid,
    pub password: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct CartItem {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
    pub unitPrice: i32,
    pub totalPrice: i64,
}
#[derive(Deserialize, Serialize)]
pub struct Cart {
    items: Vec<CartItem>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Order {
    pub orderId: i64,
    pub date: i32,
    pub id: Uuid,
    pub oder: serde_json::Value,
    pub status: String,
}

#[derive(Deserialize, Serialize)]
pub struct Singup {
    pub email: String,
    pub password: String,
    pub name: String,
}

pub type ResponseErrStr<T> = Result<(StatusCode, Json<T>), (StatusCode, String)>;

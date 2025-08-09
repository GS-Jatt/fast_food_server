#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct Menu {
    pub id: i32,
    pub name: String,
    pub unitPrice: i32,
    pub imageUrl: String,
    pub ingredients: Vec<String>,
    pub soldOut: bool,
}

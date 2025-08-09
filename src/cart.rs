use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::types::CartItem;

pub async fn update_cart(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(data): Json<Vec<CartItem>>,
) -> Result<StatusCode, StatusCode> {
    let items_json_value = serde_json::to_value(&data).map_err(|_| StatusCode::BAD_REQUEST)?;
    sqlx::query!(
        r#"INSERT INTO  cart (id, items) VALUES ($1 , $2)
         ON CONFLICT (id) DO UPDATE SET items= $2 "#,
        id,
        items_json_value
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn get_cart(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<CartItem>>), (StatusCode, String)> {
    let res = sqlx::query(r#"SELECT items FROM cart WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    match res {
        Some(row) => {
            // Extract the items JSON column and deserialize it into Vec<CartItem>
            let items: Vec<CartItem> = row
                .try_get::<serde_json::Value, _>("items")
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to parse items: {}", e),
                    )
                })
                .and_then(|json| {
                    serde_json::from_value(json).map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to deserialize items: {}", e),
                        )
                    })
                })?;
            Ok((StatusCode::OK, Json(items)))
        }
        None => Err((StatusCode::NOT_FOUND, "Cart not found".to_string())),
    }
}

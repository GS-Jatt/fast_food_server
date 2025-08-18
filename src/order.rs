use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::{PgPool, Value};
use uuid::Uuid;

use crate::types::{Order, ResponseErrStr};
pub async fn add_order(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(order): Json<Order>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query(
        r#"INSERT INTO orders (id, "orderId", date, oder, status)
        VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(id)
    .bind(order.orderId)
    .bind(order.date)
    .bind(order.oder)
    .bind(order.status)
    .execute(&db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}
pub async fn get_orders(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
) -> ResponseErrStr<Vec<Order>> {
    let res = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1 ")
        .bind(id)
        .fetch_all(&db)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    Ok((StatusCode::OK, Json(res)))
}

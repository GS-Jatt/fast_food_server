use std::env;

use crate::{
    auth::{
        login::{login, login_with_id},
        signup::signup,
    },
    cart::{get_cart, update_cart},
    menu::get_menu,
    order::{add_order, get_orders},
};

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

pub mod auth {
    pub mod login;
    pub mod signup;
}
mod cart;
pub mod menu;
mod order;
mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors_layer = CorsLayer::new()
        .allow_origin(
            "https://fast-order.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        ) // Allows requests from any origin (use with caution for public APIs)
        .allow_methods([Method::GET, Method::POST, Method::PUT]) // Allowed HTTP methods
        .allow_headers(Any); // Allows any headers

    let database_url = env::var("DATABASE_URL").expect("Database url");

    // connect to database
    let db = PgPoolOptions::new()
        .max_connections(500)
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/login/{id}", get(login_with_id))
        .route("/menu", get(get_menu))
        .route("/cart/{id}", get(get_cart).put(update_cart))
        .route("/order/{id}", get(get_orders).put(add_order))
        .layer(cors_layer)
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello, it fast food server"
}

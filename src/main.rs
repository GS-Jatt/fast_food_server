// use std::sync::Arc;

use crate::{
    auth::{
        login::{login, login_with_id},
        signup::signup,
    },
    cart::{get_cart, update_cart},
    menu::Menu,
    order::{add_order, get_orders},
};

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::cors::{Any, CorsLayer};

pub mod auth {
    pub mod login;
    pub mod signup;
}
mod cart;
pub mod menu;
mod order;
mod types;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let cors_layer = CorsLayer::new()
        .allow_origin(
            "https://fast-order.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        ) // Allows requests from any origin (use with caution for public APIs)
        .allow_methods([Method::GET, Method::POST, Method::PUT]) // Allowed HTTP methods
        .allow_headers(Any); // Allows any headers
    let db_url = secrets.get("DB_URL").unwrap();

    // connect to database
    let db = PgPoolOptions::new()
        .max_connections(500)
        .connect(&db_url)
        .await
        .expect("can't connect to database");
    // let shared_pool = Arc::new(db);
    // build our application with a single route
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

    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // println!("server on http://{}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();
    Ok(app.into())
}

async fn root() -> &'static str {
    "hello aa"
}

async fn get_menu(State(db): State<PgPool>) -> (StatusCode, Result<Json<Vec<Menu>>, String>) {
    let menu = sqlx::query_as::<_, Menu>("SELECT  * FROM menus")
        .fetch_all(&db)
        .await;

    // print!("{:?}", menu);

    match menu {
        Ok(data) => (StatusCode::OK, Ok(Json(data))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string())),
    }

    //     Menu {
    //     id: 124,
    //     name: String::from("asfasf"),
    // };
}
async fn post_foo() {}
async fn foo_bar() {}

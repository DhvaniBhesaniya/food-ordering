use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;
use crate::config::env::Config;
use crate::config::db::connect_db;
use crate::routes::{create_cart_routes, create_food_routes, create_order_routes, create_user_routes};
// use tower_http::services::ServeDir;

mod config;
mod controllers;
mod middleware;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::from_env());

    let _db_client = connect_db(config.clone()).await;

    let app = Router::new()
        .route("/test", get(handler))
        .merge(create_food_routes())
        .merge(create_user_routes())
        .merge(create_cart_routes())
        .merge(create_order_routes());
        // .nest("/images", axum::routing::get_service(ServeDir::new("./uploads")))
        // .nest("/images/user_pic", axum::routing::get_service(ServeDir::new("./uploads/user_profiles")));

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    println!("Server running on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    println!("Rust API working");
    "Rust API working.."
}

use axum::{routing::get, Router};
use configration::db::connect_db;
use tower_http::services::ServeDir;
// use models::user_model::User;
// use mongodb::bson::doc;
// use mongodb::bson::oid::ObjectId;

use crate::routes::{
    create_cart_routes, create_food_routes, create_order_routes, create_user_routes,
};
use std::net::SocketAddr;
use std::path::PathBuf;

use tokio;

mod configration;
mod controllers;
mod middleware;
mod models;
mod routes;

use crate::configration::gett;

#[tokio::main]
async fn main() {
    //       // Parse the user ID from the claims and convert to ObjectId
    //       let user_id = ObjectId::parse_str("66a095b6ba1810efd4989e90");
    //     let collection = User::get_user_collection().await;
    //     let bson_document = collection.find_one(doc! { "_id": user_id.unwrap() }).await.unwrap();
    // println!("Raw BSON document: {:?}", bson_document);

    // let config = Arc::new(Config::from_env());

    // let _db_client = connect_db(config.clone()).await;
    let _db_client = connect_db().await;

    // Define the path to the uploads directory
    let uploads_dir = PathBuf::from("uploads");
    let app = Router::new()
        .route("/test", get(handler))
        .merge(create_food_routes())
        .merge(create_user_routes())
        .merge(create_cart_routes())
        .merge(create_order_routes())
        .nest_service("/images", ServeDir::new(uploads_dir))
        .nest_service("/images/user_pic", ServeDir::new("./uploads/user_profiles"));

    let addr = SocketAddr::from(([127, 0, 0, 1], gett("port")));

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

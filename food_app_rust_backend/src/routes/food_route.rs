use axum::{
    extract::DefaultBodyLimit, routing::{get, post}, Router
};

use crate::controllers::food_controller::{add_food, list_food, remove_food};

pub fn create_food_routes() -> Router {
    Router::new()
        .route("/api/food/add", post(add_food).layer(DefaultBodyLimit::max(1024 * 1000)))
        .route("/api/food/list", get(list_food))
        .route("/api/food/remove", post(remove_food))
}

use axum::{Router, routing::get};
use crate::controllers::food_controller::{get_food};

pub fn create_food_routes() -> Router {
    Router::new().route("/api/food", get(get_food))
}

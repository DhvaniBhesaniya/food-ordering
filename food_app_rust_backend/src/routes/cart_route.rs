use axum::{Router, routing::get};
use crate::controllers::cart_controller::{get_cart};

pub fn create_cart_routes() -> Router {
    Router::new().route("/api/cart", get(get_cart))
}

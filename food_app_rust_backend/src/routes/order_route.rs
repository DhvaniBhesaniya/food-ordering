
use axum::{Router, routing::get};
use crate::controllers::order_controller::{get_order};

pub fn create_order_routes() -> Router {
    Router::new().route("/api/order", get(get_order))
}

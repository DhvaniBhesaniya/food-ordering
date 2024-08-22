use crate::{controllers::order_controller::*, middleware::auth::auth_middleware};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_order_routes() -> Router {
    Router::new()
        .route(
            "/api/order/place",
            post(place_order).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route("/api/order/verify", post(verify_order))
        .route(
            "/api/order/userorders",
            post(user_order).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route(
            "/api/order/userordershistory",
            post(user_order_history).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route("/api/order/list", get(list_orders))
        .route("/api/order/status", post(update_status))
}

use crate::controllers::cart_controller::{add_to_cart, get_cart, remove_from_cart, update_cart};
use crate::middleware::auth::auth_middleware;
use axum::{routing::post, Router};

pub fn create_cart_routes() -> Router {
    Router::new()
        .route(
            "/api/cart/add",
            post(add_to_cart).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route(
            "/api/cart/remove",
            post(remove_from_cart).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route(
            "/api/cart/update",
            post(update_cart).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route(
            "/api/cart/get",
            post(get_cart).layer(axum::middleware::from_fn(auth_middleware)),
        )
}

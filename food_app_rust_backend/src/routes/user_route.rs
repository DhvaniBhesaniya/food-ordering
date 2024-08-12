use crate::controllers::user_controller::{get_user_data, login_user, register_user};
use crate::middleware::auth::auth_middleware;
use axum::{
    extract::Form,
    http::StatusCode,
    routing::{get, post},
    Router,
};
// use tower_http::services::fs::FileSystem;
// use axum::extract::multipart::Multipart;

pub fn create_user_routes() -> Router {
    Router::new()
        .route("/api/user/register", post(register_user))
        .route("/api/user/login", post(login_user))
        .route("/api/user/userdata",get(get_user_data).layer(axum::middleware::from_fn(auth_middleware)),)
    // .route("/updateuser", post(update_user_data))
}

use axum::{
    routing::{get, post},
    Router,
    extract::Form,
    http::StatusCode,
};
use crate::controllers::user_controller::{login_user, register_user};
use crate::middleware::auth;
// use tower_http::services::fs::FileSystem;
// use axum::extract::multipart::Multipart;

pub fn create_user_routes() -> Router {
    Router::new()
        .route("/api/user/register", post(register_user))
        .route("/api/user/login", post(login_user))
        // .route("/userdata", get(get_user_data).layer(axum::middleware::from_fn(auth_middleware)))
        // .route("/updateuser", post(update_user_data))
}

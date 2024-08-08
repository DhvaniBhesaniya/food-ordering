use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse,Response},
};
use bcrypt::{hash, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::validate_email;
use crate::models::user_model::User;
use crate::config::env::Config;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    id: String,
    exp: usize,
}

pub async fn register_user(Json(payload): Json<RegisterUser>) -> impl IntoResponse {
    let collection = User::get_user_collection().await;
    let config = Config::from_env();
    let jwt_secret = &config.jwt_secret;

    // Checking if the user already exists
    if collection.find_one(doc! { "email": &payload.email }).await.unwrap().is_some() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "message": "User already exists" })));
    }

    // Validating email format & strong password
    if !validate_email(&payload.email) {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "message": "Please enter a valid email." })));
    }
    if payload.password.len() < 8 {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "message": "Password must be at least 8 characters long." })));
    }

    // Hashing the user's password
    let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();

    // Generating a dynamic customer ID
    let last_user = collection.find_one(doc! {}).sort(doc! { "customerId": -1 }).await.unwrap();
    let customer_id = if let Some(user) = last_user {
        let last_customer_number = user.customer_id.unwrap_or_else(|| "Customer0".to_string()).replace("Customer", "").parse::<usize>().unwrap();
        format!("Customer{}", last_customer_number + 1)
    } else {
        "Customer1".to_string()
    };

    let new_user = User {
        id: None,
        customer_id: Some(customer_id),
        name: payload.name,
        password: hashed_password,
        email: payload.email,
        cart_data: Some(json!({})),
        order_history: Some(Vec::new()),
        profile_img: Some("".to_string()),
    };

    collection.insert_one(new_user.clone()).await.unwrap();

    // Creating JWT token
    let claims = Claims { id: new_user.customer_id.unwrap(), exp: 3600 * 24 };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())).unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "User registered successfully",
            "token": token,
        })),
    )
}

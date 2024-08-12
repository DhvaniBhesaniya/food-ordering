use crate::config::env::Config;
use crate::middleware::auth::Claimss;
use crate::models::user_model::User;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::FindOneOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::validate_email;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct Claims {
    id: String,
    exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "customerId")]
    pub customer_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "cartData")]
    pub cart_data: serde_json::Value,
    #[serde(rename = "orderHistory")]
    pub order_history: Vec<String>,
    #[serde(rename = "profileImg")]
    pub profile_img: String,
    #[serde(rename = "__v")]
    pub version: Option<i32>,
}

pub async fn register_user(Json(payload): Json<RegisterUser>) -> impl IntoResponse {
    let collection = User::get_user_collection().await;
    let config = Config::from_env();
    let jwt_secret = &config.jwt_secret;

    // Checking if the user already exists
    if collection
        .find_one(doc! { "email": &payload.email })
        .await
        .unwrap()
        .is_some()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "User already exists" })),
        );
    }

    // Validating email format & strong password
    if !validate_email(&payload.email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "Please enter a valid email." })),
        );
    }
    if payload.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(
                json!({ "success": false, "message": "Password must be at least 8 characters long." }),
            ),
        );
    }

    // Hashing the user's password
    let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();

    // Generating a dynamic customer ID
    //  let find_options = FindOneOptions::builder()
    //  .sort(doc! { "customerId": -1 })
    //  .build();

    let last_user_doc = collection
        .find_one(doc! {})
        .sort(doc! { "customerId": -1 })
        .await
        .unwrap();

    let customer_id = if let Some(user_doc) = last_user_doc {
        // Extract the last customer ID from the BSON document
        let last_customer_id = user_doc.get_str("customerId").unwrap_or("Customer0");
        let last_customer_number = last_customer_id
            .replace("Customer", "")
            .parse::<usize>()
            .unwrap();
        format!("Customer{}", last_customer_number + 1)
    } else {
        "Customer1".to_string()
    };

    // Constructing the new user BSON document
    let new_user_doc = doc! {
        "customerId": customer_id.clone(),
        "name": &payload.name,
        "password": hashed_password,
        "email": &payload.email,
        "cartData": doc! {},
        "orderHistory": Vec::<ObjectId>::new(),
        "profileImg": "",
    };

    // Inserting the new user into the database
    collection.insert_one(new_user_doc.clone()).await.unwrap();

    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(3600 * 24))
        .expect("valid timestamp")
        .timestamp();

    // Creating JWT token
    let claims = Claims {
        id: customer_id,
        exp: expiration_time,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "User registered successfully",
            "token": token,
        })),
    )
}

pub async fn login_user(Json(payload): Json<LoginUser>) -> impl IntoResponse {
    let collection = User::get_user_collection().await;
    let config = Config::from_env();
    let jwt_secret = &config.jwt_secret;

    // Checking if the user exists
    let user_doc = match collection
        .find_one(doc! { "email": &payload.email })
        .await
        .unwrap()
    {
        Some(doc) => doc,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "success": false, "message": "User does not exist" })),
            );
        }
    };

    // Verifying password
    let stored_password = user_doc.get_str("password").unwrap();
    if !verify(&payload.password, stored_password).unwrap() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "success": false, "message": "Invalid Password" })),
        );
    }

    // Extract the user ID from the document
    let user_id = user_doc.get_object_id("_id").unwrap().to_hex();

    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(3600 * 24))
        .expect("valid timestamp")
        .timestamp();

    // Creating JWT token
    let claims = Claims {
        id: user_id,
        exp: expiration_time,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Login successful",
            "token": token,
        })),
    )
}

pub async fn get_user_data(Extension(claims): Extension<Claimss>) -> impl IntoResponse {
   
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            );
        }
    };

    let collection = User::get_user_collection().await;
    match collection.find_one(doc! { "_id": user_id }).await {
        Ok(Some(user_doc)) => {
            // Manually extract and transform the data
            let id = user_doc
                .get_object_id("_id")
                .map(|oid| oid.to_hex())
                .unwrap_or_default();

            let customer_id = user_doc
                .get_str("customerId")
                .unwrap_or_default()
                .to_string();
            let name = user_doc.get_str("name").unwrap_or_default().to_string();
            let email = user_doc.get_str("email").unwrap_or_default().to_string();
            let password = user_doc.get_str("password").unwrap_or_default().to_string();
            let cart_data = user_doc.get("cartData").cloned().unwrap_or_default().into();

            let order_history = user_doc
                .get_array("orderHistory")
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|o| o.as_object_id().map(|oid| oid.to_hex()))
                .collect();

            let profile_img = user_doc
                .get_str("profileImg")
                .unwrap_or_default()
                .to_string();
            let version = user_doc.get_i32("__v").ok();

            // Create UserResponse
            let user_response = UserResponse {
                id,
                customer_id,
                name,
                email,
                password,
                cart_data,
                order_history,
                profile_img,
                version,
            };

            // Return the user data
            (
                StatusCode::OK,
                Json(json!({ "success": true, "data": user_response })),
            )
        }
        Ok(None) => {
            // User not found
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "message": "User not found" })),
            )
        }
        Err(e) => {
            // Error during database lookup
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": e.to_string() })),
            )
        }
    }
}

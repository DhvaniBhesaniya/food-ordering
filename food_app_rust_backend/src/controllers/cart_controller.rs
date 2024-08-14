use crate::models::user_model::User;
use crate::middleware::auth::Claimss;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};

use mongodb::bson::Bson;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Struct for the CartItem payload
#[derive(Debug, Deserialize, Serialize)]
pub struct CartItem {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

// // Add items to user cart
pub async fn add_to_cart(
    Extension(claims): Extension<Claimss>, // User ID extracted from middleware
    Json(payload): Json<CartItem>,
) -> impl IntoResponse {
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            );
        }
    };

    let filter = doc! { "_id": user_id };
    let collection = User::get_user_collection().await;

    // Find the user document
    match collection.find_one(filter.clone()).await {
        Ok(Some(user_doc)) => {
            let mut cart_data = match user_doc.get("cartData") {
                Some(Bson::Document(d)) => d.clone(),
                _ => doc! {},
            };

            let item_id = payload.item_id.clone();
            let count = cart_data
                .get(&item_id)
                .and_then(|bson| bson.as_i32())
                .unwrap_or(0)
                + 1;

            cart_data.insert(item_id, Bson::Int32(count));

            // Update the user's cart data
            match collection
                .update_one(filter, doc! { "$set": { "cartData": cart_data } })
                .await
            {
                Ok(_) => (
                    StatusCode::OK,
                    Json(json!({ "success": true, "message": "Item added to cart" })),
                ),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "message": "Failed to update cart" })),
                ),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "message": "User not found" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Database error" })),
        ),
    }
}

// Remove items from user cart
pub async fn remove_from_cart(
    Extension(claims): Extension<Claimss>, // User ID extracted from middleware
    Json(payload): Json<CartItem>,
) -> impl IntoResponse {
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            );
        }
    };

    let filter = doc! { "_id": user_id };
    let collection = User::get_user_collection().await;

    // Find the user document
    match collection.find_one(filter.clone()).await {
        Ok(Some(user_doc)) => {
            let mut cart_data = match user_doc.get("cartData") {
                Some(Bson::Document(d)) => d.clone(),
                _ => doc! {},
            };

            let item_id = payload.item_id.clone();
            if let Some(Bson::Int32(count)) = cart_data.get_mut(&item_id) {
                let mut count = *count;
                count -= 1;
                if count <= 0 {
                    cart_data.remove(&item_id);
                } else {
                    cart_data.insert(item_id, Bson::Int32(count));
                }

                // Update the user's cart data
                match collection
                    .update_one(filter, doc! { "$set": { "cartData": cart_data } })
                    .await
                {
                    Ok(_) => (
                        StatusCode::OK,
                        Json(json!({ "success": true, "message": "Item removed from cart" })),
                    ),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "success": false, "message": "Failed to update cart" })),
                    ),
                }
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "success": false, "message": "Item not found in cart" })),
                )
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "message": "User not found" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Database error" })),
        ),
    }
}

// Update the item quantity in the cart
pub async fn update_cart(
    Extension(claims): Extension<Claimss>, // User ID extracted from middleware
    Json(payload): Json<CartItem>,
) -> impl IntoResponse {
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            );
        }
    };

    let filter = doc! { "_id": user_id };
    let collection = User::get_user_collection().await;

    // Find the user document
    match collection.find_one(filter.clone()).await {
        Ok(Some(user_doc)) => {
            let mut cart_data = match user_doc.get("cartData") {
                Some(Bson::Document(d)) => d.clone(),
                _ => doc! {},
            };

            let item_id = payload.item_id.clone();
            let quantity = payload.quantity.unwrap();

            if quantity <= 0 {
                cart_data.remove(&item_id);
            } else {
                cart_data.insert(item_id, Bson::Int32(quantity));
            }

            // Update the user's cart data
            match collection
                .update_one(filter, doc! { "$set": { "cartData": cart_data } })
                .await
            {
                Ok(_) => (
                    StatusCode::OK,
                    Json(json!({ "success": true, "message": "Cart updated" })),
                ),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "message": "Failed to update cart" })),
                ),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "message": "User not found" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Database error" })),
        ),
    }
}

// Fetch user cart data
pub async fn get_cart(
    Extension(claims): Extension<Claimss>, // User ID extracted from middleware
) -> impl IntoResponse {
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(oid) => oid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Invalid user ID" })),
            );
        }
    };

    let filter = doc! { "_id": user_id };
    let collection = User::get_user_collection().await;
    match collection.find_one(filter).await {
        Ok(Some(user_doc)) => {
            let cart_data = user_doc.get("cartData").cloned().unwrap();
            (
                StatusCode::OK,
                Json(json!({ "success": true, "cartData": cart_data })),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "message": "User not found" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Failed to fetch user data" })),
        ),
    }
}

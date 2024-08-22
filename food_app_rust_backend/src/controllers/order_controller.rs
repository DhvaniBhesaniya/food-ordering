use crate::configration::gett;
use crate::middleware::auth::Claimss;
use crate::models::{order_model::Order, user_model::User};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use mongodb::bson::{self, doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stripe::{Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, Currency};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Address {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: String,
    street: String,
    city: String,
    state: String,
    #[serde(rename = "zipCode")]
    zip_code: String,
    country: String,
    phone: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    address: Address,
    items: Vec<Item>,
    amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    _id: String,
    name: String,
    price: f64,
    quantity: u32,
    category: String,
    description: String,
    image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OrderDocument {
    _id: ObjectId,
    #[serde(rename = "userId")]
    user_id: String,
    items: Vec<Item>,
    amount: f64,
    address: Address,
    status: String,
    payment: bool,
    date: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct VerifyOrderPayload {
    #[serde(rename = "orderId")]
    order_id: String,
    success: String,
}

#[derive(Deserialize)]
pub struct UpdateStatusPayload {
    #[serde(rename = "orderId")]
    order_id: String,
    status: String,
}

// Placing the order
pub async fn place_order(
    Extension(claims): Extension<Claimss>,
    Json(req): Json<PlaceOrderRequest>,
) -> impl IntoResponse {
    let local_frontend_url = gett::<String>("local_frontend_url");
    // let deployed_frontend_url = gett::<String>("deployed_frontend_url");
    let frontend_url = local_frontend_url;

    let stripe_key = gett::<String>("stripe_secret_key");
    let client = Client::new(stripe_key);

    let order_collection: Collection<Document> = Order::get_order_collection().await;

    let new_order = OrderDocument {
        _id: ObjectId::new(),
        user_id: ObjectId::parse_str(&claims.id).unwrap().to_hex(),
        items: req.items.clone(),
        amount: req.amount,
        address: req.address.clone(),
        status: "Food Processing".to_string(),
        payment: false,
        date: Utc::now(),
    };

    // Convert OrderDocument to BSON Document
    let bson_order = bson::to_document(&new_order).unwrap();

    // Insert the order into the collection
    match order_collection.insert_one(bson_order).await {
        Ok(_) => {
            let user_collection: Collection<Document> = User::get_user_collection().await;
            if let Err(_err) = user_collection
                .update_one(
                    doc! { "_id": &ObjectId::parse_str(&claims.id).unwrap() },
                    doc! { "$set": { "cartData": bson::to_bson(&[] as &[Item]).unwrap() }},
                )
                .await
            {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "message": "Error updating user cart data" })),
                );
            }

            let mut line_items: Vec<CreateCheckoutSessionLineItems> = req
                .items
                .iter()
                .map(|item| {
                    CreateCheckoutSessionLineItems {
                        price_data: Some(stripe::CreateCheckoutSessionLineItemsPriceData {
                            currency: Currency::INR,
                            product_data: Some(
                                stripe::CreateCheckoutSessionLineItemsPriceDataProductData {
                                    name: item.name.clone(),
                                    ..Default::default()
                                },
                            ),
                            unit_amount: Some((item.price * 100.0 * 80.0) as i64), // Convert to smallest currency unit
                            ..Default::default()
                        }),
                        quantity: Some(item.quantity.into()),
                        ..Default::default()
                    }
                })
                .collect();

            // Add delivery charge
            line_items.push(CreateCheckoutSessionLineItems {
                price_data: Some(stripe::CreateCheckoutSessionLineItemsPriceData {
                    currency: Currency::INR,
                    product_data: Some(
                        stripe::CreateCheckoutSessionLineItemsPriceDataProductData {
                            name: "Delivery Charge".to_string(),
                            ..Default::default()
                        },
                    ),
                    unit_amount: Some((2.0 * 100.0 * 10.0) as i64), // Delivery charge   20 rs delivery charge  -> 2% * 100 * 10
                    ..Default::default()
                }),
                quantity: Some(1),
                ..Default::default()
            });

            let session_url =
                create_stripe_session(&client, line_items, frontend_url, new_order._id.to_string())
                    .await;

            (
                StatusCode::OK,
                Json(json!({ "success": true, "session_url": session_url })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Error placing order" })),
        ),
    }
}

async fn create_stripe_session(
    client: &Client,
    line_items: Vec<CreateCheckoutSessionLineItems>,
    frontend_url: String,
    new_order_id: String,
) -> String {
    let cancel_url = format!(
        "{}/verify?success=false&orderId={}",
        frontend_url, new_order_id
    );
    let success_url = format!(
        "{}/verify?success=true&orderId={}",
        frontend_url, new_order_id
    );

    let checkout_session = {
        let mut params = stripe::CreateCheckoutSession::new();
        params.cancel_url = Some(cancel_url.as_str());
        params.success_url = Some(success_url.as_str());
        params.mode = Some(stripe::CheckoutSessionMode::Payment);
        params.line_items = Some(line_items);
        params.expand = &["line_items", "line_items.data.price.product"];

        stripe::CheckoutSession::create(client, params)
            .await
            .unwrap()
    };

    checkout_session.url.unwrap()
}

pub async fn verify_order(
    Json(payload): Json<VerifyOrderPayload>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let VerifyOrderPayload { order_id, success } = payload;

    // Convert the order_id to ObjectId
    let object_id = match ObjectId::parse_str(&order_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Get the orders collection
    let orders_collection = Order::get_order_collection().await;

    // Find the order by its ID
    let filter = doc! { "_id": object_id };
    let order_exists = orders_collection
        .find_one(filter.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if the order exists
    if order_exists.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // If success is true, update the payment field
    if success == "true" {
        let update_doc = doc! { "$set": { "payment": true } };
        orders_collection
            .update_one(filter, update_doc)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(
            json!({ "success": true, "message": "Order has been paid" }),
        ))
    } else {
        // If success is false, delete the order
        orders_collection
            .delete_one(filter)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(
            json!({ "success": false, "message": "Order has been deleted due to payment failure" }),
        ))
    }
}

pub async fn user_order(
    Extension(claims): Extension<Claimss>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = ObjectId::parse_str(&claims.id).unwrap();
    let orders_collection = Order::get_order_collection().await;

    // Retrieve all orders for the user
    let mut cursor = orders_collection
        .find(doc! { "userId": &user_id.to_hex() })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut orders = Vec::new();

    // Iterate over the cursor to collect and filter the orders
    while let Some(result) = cursor.next().await {
        match result {
            Ok(mut doc) => {
                if let Some(status) = doc.get_str("status").ok() {
                    // Filter out orders with the status "Delivered"
                    if status != "Delivered" {
                        if let Some(id) = doc.remove("_id") {
                            if let Some(object_id) = id.as_object_id() {
                                doc.insert("_id", object_id.to_string());
                            }
                        }
                        orders.push(doc);
                    }
                }
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    Ok(Json(json!({ "success": true, "data": orders })))
}

pub async fn user_order_history(
    Extension(claims): Extension<Claimss>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = ObjectId::parse_str(&claims.id).unwrap();
    let users_collection = User::get_user_collection().await;

    // Fetch the user document
    let user_doc = users_collection
        .find_one(doc! { "_id": user_id })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user_doc) = user_doc {
        // Extract the order history from the user document
        if let Some(order_history) = user_doc.get_array("orderHistory").ok() {
            let order_ids: Vec<ObjectId> = order_history
                .iter()
                .filter_map(|id| id.as_object_id())
                .collect();

            let orders_collection = Order::get_order_collection().await;

            // Fetch all orders corresponding to the order IDs
            let mut cursor = orders_collection
                .find(doc! { "_id": { "$in": order_ids } })
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let mut orders = Vec::new();

            // Iterate over the cursor to collect the orders
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(mut doc) => {
                        if let Some(id) = doc.remove("_id") {
                            if let Some(object_id) = id.as_object_id() {
                                doc.insert("_id", object_id.to_string());
                            }
                        }
                        orders.push(doc)
                    }
                    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }

            Ok(Json(json!({ "success": true, "data": orders })))
        } else {
            Ok(Json(json!({ "success": true, "data": [] })))
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn list_orders() -> (StatusCode, Json<serde_json::Value>) {
    let collection = Order::get_order_collection().await;

    match collection.find(doc! {}).await {
        Ok(mut cursor) => {
            let mut orders = vec![];

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(mut doc) => {
                        // Convert _id field to string
                        if let Some(id) = doc.remove("_id") {
                            if let Some(object_id) = id.as_object_id() {
                                doc.insert("_id", object_id.to_string());
                            }
                        }

                        // Push the order to the vector
                        orders.push(doc);
                    }
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "success": false, "message": "Error reading cursor" })),
                        );
                    }
                }
            }

            (
                StatusCode::OK,
                Json(json!({ "success": true, "data": orders })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Failed to retrieve orders" })),
        ),
    }
}

pub async fn update_status(
    Json(payload): Json<UpdateStatusPayload>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let UpdateStatusPayload { order_id, status } = payload;

    let object_id = ObjectId::parse_str(&order_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let orders_collection = Order::get_order_collection().await;

    // Update the order status
    let update_doc = doc! { "$set": { "status": &status } };
    orders_collection
        .update_one(
            doc! { "_id": object_id },
            update_doc,
        )
        .await
        .map_err(|err| {
            println!("Error during update_one: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Retrieve the updated order document
    let updated_order_doc = orders_collection
        .find_one(doc! { "_id": object_id })
        .await
        .map_err(|err| {
            println!("Error retrieving updated order: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(updated_order_doc) = updated_order_doc {
        // Get the user ID as a string, then convert it to ObjectId
        let user_id_str = updated_order_doc
            .get_str("userId")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let user_id = ObjectId::parse_str(user_id_str).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if status == "Delivered" {
            let users_collection = User::get_user_collection().await;
            let update_user_doc = doc! { "$addToSet": { "orderHistory": object_id } };

            let user_update_result = users_collection
                .update_one(doc! { "_id": user_id }, update_user_doc)
                .await
                .map_err(|err| {
                    println!("Error updating user order history: {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            if user_update_result.matched_count == 0 {
                return Err(StatusCode::NOT_FOUND);
            }

            return Ok(Json(json!({
                "success": true,
                "message": "Order delivered and moved to user's history",
                "data": updated_order_doc
            })));
        }

        Ok(Json(json!({
            "success": true,
            "message": "Order status updated successfully",
            "data": updated_order_doc
        })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
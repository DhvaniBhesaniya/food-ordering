use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use crate::configration::gett;
// use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "cartData", skip_serializing_if = "Option::is_none")]
    pub cart_data: Option<serde_json::Value>,
    #[serde(rename = "orderHistory", skip_serializing_if = "Option::is_none")]
    pub order_history: Option<Vec<ObjectId>>,
    #[serde(rename = "profileImg", skip_serializing_if = "Option::is_none")]
    pub profile_img: Option<String>,
}

impl User {
    pub async fn get_user_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database("food-delivery").collection::<Document>("users")
    }
}

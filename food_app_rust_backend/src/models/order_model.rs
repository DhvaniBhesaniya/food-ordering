use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::configration::gett;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub items: Vec<serde_json::Value>, // Assuming items are stored as an array of Documents
    pub amount: f64,
    pub address: serde_json::Value, // Assuming address is an embedded document
    pub status: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds_option", rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    pub payment: Option<bool>,
}

impl Order {
    pub async fn get_order_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database("food-delivery").collection::<Document>("orders")
    }
}

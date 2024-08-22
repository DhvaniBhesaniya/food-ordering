use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use crate::configration::gett;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub items: Vec<serde_json::Value>,
    pub amount: f64,
    pub address: serde_json::Value,
    pub status: String,
    pub date: DateTime<Utc>,
    pub payment: bool,
}

impl Order {
    pub async fn get_order_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client
            .database("food-delivery")
            .collection::<Document>("orders")
    }
}

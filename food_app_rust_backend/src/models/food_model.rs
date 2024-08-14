use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use crate::configration::gett;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Food {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub image: String,
    pub category: String,
}

impl Food {
    pub async fn get_food_collection() -> Collection<Document> {
        let url = gett::<String>("mongodb_url");
        let client_options = ClientOptions::parse(url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client.database("food-delivery").collection::<Document>("foods")
    }
}

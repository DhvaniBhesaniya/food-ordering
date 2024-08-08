use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use crate::config::env::Config;

pub async fn connect_db(config: Arc<Config>) -> Client {
    let client_options = ClientOptions::parse(&config.mongodb_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    println!("DB connected");
    client
}

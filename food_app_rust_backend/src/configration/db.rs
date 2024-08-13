use crate::configration::gett;
use mongodb::{options::ClientOptions, Client};


pub async fn connect_db() -> Client {
    let url = gett::<String>("mongodb_url");
    let client_options = ClientOptions::parse(url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    println!("DB connected");
    client
}

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::json;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;
// use futures_util::stream::StreamExt;
use futures_util::StreamExt;


use crate::models::food_model::Food;

#[derive(Debug, serde::Deserialize)]
pub struct FoodRemoval {
    pub id: String,
}

pub async fn add_food(
    mut multipart: Multipart,
) -> impl IntoResponse {
    let upload_dir = PathBuf::from("uploads");
    if !upload_dir.exists() {
        fs::create_dir_all(&upload_dir).unwrap();
    }

    let mut food_data = doc! {};
    let mut food_image = None;

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("image") => {
                if let Some(file_name) = field.file_name() {
                    let file_name = format!("{}{}", Uuid::new_v4(), file_name);
                    let file_path = upload_dir.join(file_name.clone());
                    let mut file = File::create(&file_path).unwrap();
                    while let Some(chunk) = field.chunk().await.unwrap() {
                        file.write_all(&chunk).unwrap();
                    }
                    food_image = Some(file_name);
                }
            }
            Some("name") => {
                let value = field.text().await.unwrap();
                food_data.insert("name", value);
            }
            Some("description") => {
                let value = field.text().await.unwrap();
                food_data.insert("description", value);
            }
            Some("price") => {
                let value = field.text().await.unwrap().parse::<i32>().unwrap();
                food_data.insert("price", value);
            }
            Some("category") => {
                let value = field.text().await.unwrap();
                food_data.insert("category", value);
            }
            _ => {}
        }
    }

    if let Some(image) = food_image {
        food_data.insert("image", image);
    }

    let collection = Food::get_food_collection().await;
    match collection.insert_one(food_data).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "success": true, "message": "Food Added" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Failed to add food" })),
        ),
    }
}


pub async fn list_food() -> impl IntoResponse {
    let collection = Food::get_food_collection().await;
    match collection.find(doc! {}).await {
        Ok(mut cursor) => {
            let mut foods = vec![];

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(mut doc) => {
                        // Convert _id field to string
                        if let Some(id) = doc.remove("_id") {
                            if let Some(object_id) = id.as_object_id() {
                                doc.insert("_id", object_id.to_string());
                            }
                        }
                        foods.push(doc);
                    }
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "success": false, "message": "Error processing document" })),
                        );
                    }
                }
            }

            (
                StatusCode::OK,
                Json(json!({ "success": true, "data": foods })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Failed to retrieve food items" })),
        ),
    }
}

pub async fn remove_food(
    Json(payload): Json<FoodRemoval>,
) -> impl IntoResponse {
    let collection = Food::get_food_collection().await;
    let filter = doc! { "_id": ObjectId::parse_str(&payload.id).unwrap() };

    match collection.find_one(filter.clone()).await {
        Ok(Some(food_doc)) => {
            if let Some(image) = food_doc.get_str("image").ok() {
                let file_path = PathBuf::from("uploads").join(image);
                if file_path.exists() {
                    fs::remove_file(file_path).unwrap();
                }
            }

            match collection.delete_one(filter).await {
                Ok(_) => (
                    StatusCode::OK,
                    Json(json!({ "success": true, "message": "Food Removed" })),
                ),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "message": "Failed to remove food" })),
                ),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "message": "Food not found" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Failed to fetch food item" })),
        ),
    }
}

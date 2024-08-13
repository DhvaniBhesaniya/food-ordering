use crate::models::user_model::User;
use crate::{configration, middleware::auth::Claimss};
use axum::{
    extract::{Extension, Json, Multipart},
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

use std::fs::{self, File};
use std::io::Write;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub current_password: Option<String>,
    pub phone_number: Option<String>,
    pub new_password: Option<String>,
}

pub async fn register_user(Json(payload): Json<RegisterUser>) -> impl IntoResponse {
    let collection = User::get_user_collection().await;

    let jwt_secret = configration::gett::<String>("jwt_secret");

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

    let jwt_secret = configration::gett::<String>("jwt_secret");

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


pub async fn update_user_data(
    Extension(claims): Extension<Claimss>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Initialize the user collection
    let collection = User::get_user_collection().await;
    let user_id = match ObjectId::parse_str(&claims.id) {
        Ok(id) => id,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "Invalid user ID" })),
        ),
    };

    // Get the user's document
    let filter = doc! { "_id": user_id };
    let user_doc = match collection.find_one(filter.clone()).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "message": "User not found" })),
        ),
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "Failed to fetch user data" })),
        ),
    };

    // Prepare fields for update
    let mut update_fields = doc! {};
    let mut profile_img_name = None;
    let mut current_password = None;
    let mut new_password = None;

    // Process form fields and file upload
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("profileImg") => {
                if let Some(file_name) = field.file_name() {
                    let file_name = file_name.to_string();
                    
                    // Define the user-specific folder
                    let user_folder = format!("uploads/user_profiles/{}", user_doc.get_str("customerId").unwrap());
                    let user_folder_path = PathBuf::from(&user_folder);

                    // Create the folder if it doesn't exist
                    if !user_folder_path.exists() {
                        std::fs::create_dir_all(&user_folder_path).unwrap();
                    }

                    // Save the file
                    let file_path = user_folder_path.join(&file_name);
                    let mut file = File::create(&file_path).unwrap();
                    while let Some(chunk) = field.chunk().await.unwrap() {
                        file.write_all(&chunk).unwrap();
                    }

                    profile_img_name = Some(file_name);
                }
            }
            Some("name") => {
                let name_value = field.text().await.unwrap();
                update_fields.insert("name", name_value);
            }
            Some("email") => {
                let email_value = field.text().await.unwrap();
                update_fields.insert("email", email_value);
            }
            Some("phoneNumber") => {
                let phone_value = field.text().await.unwrap();
                update_fields.insert("phoneNumber", phone_value);
            }
            Some("currentPassword") => {
                current_password = Some(field.text().await.unwrap());
            }
            Some("newPassword") => {
                new_password = Some(field.text().await.unwrap());
            }
            _ => {}
        }
    }

    // Handle password change
    if let (Some(current), Some(new)) = (current_password, new_password) {
        if current.is_empty() || new.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Current password and new password cannot be empty" })),
            );
        }
        let stored_password = user_doc.get_str("password").unwrap();
        if verify(&current, stored_password).unwrap() {
            let hashed_password = hash(new, DEFAULT_COST).unwrap();
            update_fields.insert("password", hashed_password);
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "message": "Current password is incorrect" })),
            );
        }
    }

    // Set profile image if provided
    if let Some(img_name) = profile_img_name {
        update_fields.insert("profileImg", img_name);
    }

    // Update the user document in MongoDB
    if !update_fields.is_empty() {
        let update_doc = doc! { "$set": update_fields };
        if collection.update_one(filter, update_doc).await.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "Failed to update user data" })),
            );
        }
    }

    // Return a success response
    (
        StatusCode::OK,
        Json(json!({ "success": true, "message": "User details updated successfully" })),
    )
}

// pub async fn update_user_data(
//     Extension(claims): Extension<Claimss>,
//     mut multipart: Multipart,
//     // Json(payload): Json<UpdateUserData>,
// ) -> impl IntoResponse {
//     // Initialize the user collection
//     let collection = User::get_user_collection().await;

//     // Parse user ID
//     let user_id = match mongodb::bson::oid::ObjectId::parse_str(&claims.id) {
//         Ok(id) => id,
//         Err(_) => return (
//             StatusCode::BAD_REQUEST,
//             Json(json!({ "success": false, "message": "Invalid user ID" })),
//         ),
//     };

//     // Get the user's document
//     let filter = doc! { "_id": user_id };
//     let user_doc = match collection.find_one(filter.clone()).await {
//         Ok(Some(doc)) => doc,
//         Ok(None) => return (
//             StatusCode::NOT_FOUND,
//             Json(json!({ "success": false, "message": "User not found" })),
//         ),
//         Err(_) => return (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({ "success": false, "message": "Failed to fetch user data" })),
//         ),
//     };

//     // Handle file upload
//     let mut profile_img_name = None;
//     while let Some(mut field) = match multipart.next_field().await {
//         Ok(f) => f,
//         Err(_) => return (
//             StatusCode::BAD_REQUEST,
//             Json(json!({ "success": false, "message": "Failed to read multipart data" })),
//         ),
//     } {
//         if field.name() == Some("profileImg") {
//             let file_name = match field.file_name() {
//                 Some(name) => name.to_string(),
//                 None => return (
//                     StatusCode::BAD_REQUEST,
//                     Json(json!({ "success": false, "message": "Missing file name" })),
//                 ),
//             };

//             // Define the user-specific folder
//             let user_folder = format!("uploads/user_profiles/{}", user_doc.get_str("customerId").unwrap_or(&"default".to_string()));
//             let user_folder_path = PathBuf::from(&user_folder);

//             // Create the folder if it doesn't exist
//             if !user_folder_path.exists() {
//                 if let Err(err) = fs::create_dir_all(&user_folder_path) {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({ "success": false, "message": format!("Failed to create directory: {}", err) })),
//                     );
//                 }
//             }

//             // Save the file
//             let file_path = user_folder_path.join(&file_name);
//             let mut file = match File::create(&file_path) {
//                 Ok(f) => f,
//                 Err(err) => return (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({ "success": false, "message": format!("Failed to create file: {}", err) })),
//                 ),
//             };
//             while let Some(chunk) = match field.chunk().await {
//                 Ok(c) => c,
//                 Err(_) => return (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({ "success": false, "message": "Failed to read file chunk" })),
//                 ),
//             } {
//                 if let Err(err) = file.write_all(&chunk) {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({ "success": false, "message": format!("Failed to write to file: {}", err) })),
//                     );
//                 }
//             }

//             // Update the user document with the new profile image name
//             profile_img_name = Some(file_name);
//         }
//     }

//     // Prepare update fields
//     let mut update_fields = doc! {};

//     // Uncomment and handle the payload fields if needed
//     // if let Some(name) = payload.name {
//     //     update_fields.insert("name", name);
//     // }

//     // if let Some(email) = payload.email {
//     //     update_fields.insert("email", email);
//     // }

//     // if let Some(current_password) = payload.current_password {
//     //     if let Some(new_password) = payload.new_password {
//     //         // Verify the current password
//     //         let stored_password = match user_doc.get_str("password") {
//     //             Ok(p) => p,
//     //             Err(_) => return (
//     //                 StatusCode::BAD_REQUEST,
//     //                 Json(json!({ "success": false, "message": "Password verification failed" })),
//     //             ),
//     //         };
//     //         if bcrypt::verify(&current_password, stored_password).map_err(|_| UpdateUserError::PasswordVerificationError)? {
//     //             // Hash the new password
//     //             let hashed_password = bcrypt::hash(new_password, bcrypt::DEFAULT_COST).map_err(|_| UpdateUserError::PasswordHashingError)?;
//     //             update_fields.insert("password", hashed_password);
//     //         } else {
//     //             return (
//     //                 StatusCode::BAD_REQUEST,
//     //                 Json(json!({ "success": false, "message": "Current password is incorrect" })),
//     //             );
//     //         }
//     //     }
//     // }

//     if let Some(profile_img_name) = profile_img_name {
//         update_fields.insert("profileImg", profile_img_name);
//     }

//     // Update the user document in MongoDB
//     if !update_fields.is_empty() {
//         if let Err(_) = collection.update_one(filter, doc! { "$set": update_fields }).await {
//             return (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({ "success": false, "message": "Failed to update user document" })),
//             );
//         }
//     }

//     // Return a success response
//     (StatusCode::OK, Json(json!({ "success": true, "message": "User details updated successfully" })))
// }
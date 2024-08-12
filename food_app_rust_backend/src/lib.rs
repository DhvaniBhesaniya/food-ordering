// use axum::extract::rejection::JsonRejection;
// use axum::extract::Multipart;
// use axum::http::response;
// use axum::response::{IntoResponse, Response};
// use axum::Json;
// use axum::{

//     http::StatusCode,
//     routing::post,
//     Router,
// };
// use bytes::Bytes;
// use std::fs::File;
// use std::io::Write;
// use std::net::SocketAddr;
// use std::path::{Path, PathBuf};

// async fn upload_image(mut multipart: Multipart) -> impl IntoResponse {
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap_or("unnamed").to_string();
//         let filename = field.file_name().unwrap_or("unnamed").to_string();
//         let data: Bytes = field.bytes().await.unwrap();

//         // Use a relative path instead of an absolute path
//         let upload_dir = PathBuf::from("uploads");

//         // Ensure the upload directory exists
//         if !upload_dir.exists() {
//             std::fs::create_dir_all(&upload_dir).unwrap();
//         }

//         let file_path = upload_dir.join(&filename);

//         // Handle file creation and writing with proper error handling
//         match File::create(&file_path) {
//             Ok(mut file) => {
//                 if let Err(e) = file.write_all(&data) {
//                     eprintln!("Failed to write to file: {}", e);
//                     return format!("Failed to save file: {}", e).into_response();
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Failed to create file: {}", e);
//                 return format!("Failed to create file: {}", e).into_response();
//             }
//         }

//         println!("Uploaded {} as {}", name, filename);
//     }

//     "Upload successful".to_string().into_response()
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/upload", post(upload_image));

//     let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

//     axum::Server::bind(&addr).serve(app.clone().into_make_service()).await.expect("something went wrong!");
// }
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::Deserialize;

use crate::configration;

#[derive(Debug, Deserialize, Clone)]
pub struct Claimss {
    pub id: String,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract JWT token from headers
    let token = req
        .headers()
        .get("token")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    

    // Read the secret from config
  
    let jwt_secret = configration::gett::<String>("jwt_secret");
    
    // Verify and decode token
    let token_data: TokenData<Claimss> = match decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Token decode error: {:?}", err); // Add this line to log the error
            return Err(StatusCode::UNAUTHORIZED);
        }
    };



    // Attach the user ID to the request body
    req.extensions_mut().insert(token_data.claims);

    // Proceed to the next middleware or handler
    Ok(next.run(req).await)
}

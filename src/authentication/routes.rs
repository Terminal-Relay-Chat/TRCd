use axum::{http::StatusCode, response::IntoResponse, extract::Json};
use serde::{Serialize, Deserialize};
use serde_json::json;
use log::warn;

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    handle: String,
    password: String,
}

/// Route to log a User in and return a JWT
pub async fn login(Json(raw_user): Json<LoginRequest>) -> impl IntoResponse {

    // validate fields
    if raw_user.handle.is_empty() {return Err((StatusCode::BAD_REQUEST, "{\"error\": true, \"value\": \"field \\\"handle\\\" cannot be null\"}"));}
    if raw_user.password.is_empty() {return Err((StatusCode::BAD_REQUEST, "{\"error\": true, \"value\": \"field \\\"password\\\" cannot be null\"}"));}
    
    // find the user on the database, if they don't exist return a BAD_REQUEST and warn!
    //TODO: use database to see if the user is valid
    let user = User {
        user_type: super::user::UserMode::User,
        username: raw_user.handle.clone(), // TODO: change from database
        permission_level: super::user::UserPermissions::User, // TODO: change from database
        handle: raw_user.handle,
        provider_site: None, // TODO: change from database
        banned: false, // TODO: change from database
        uid: 67, // TODO: change from database
    };

    // return a jwt
    let token = match super::token::create_token(user.clone(), None) {
        Err(e) => {
            warn!("error creating token: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error. Please report to an admin (probably via email...)"))
        }
        Ok(token) => token,
    };
    
    Ok(json!({"error": false, "token": token}).to_string())
}

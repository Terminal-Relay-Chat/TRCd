use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, Json, State}, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use crate::{backend::server::{APIResponse, APIState}, database::database::DBCalls};
use serde_json::json;
use log::warn;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    handle: String,
    password: String,
}

/// Route to log a User in and return a JWT
#[axum::debug_handler]
pub async fn login(State(state): State<APIState>, Json(body): Json<LoginRequest>) -> Result<String, (StatusCode, String)> {

    // validate fields
    if body.handle.is_empty() {return Err((StatusCode::BAD_REQUEST, APIResponse::new(true, "field \"handle\" cannot be empty").serialize()))}
    if body.password.is_empty() {return Err((StatusCode::BAD_REQUEST, APIResponse::new(true, "field \"password\" cannot be empty").serialize()))}
    
    // find the user on the database, if they don't exist return a BAD_REQUEST and warn!
    
    let user_entry = match state.db.fetch_user(&body.handle).await {
        Ok(v) => v,
        Err(message) => {
            warn!("{}", message);
            return Err((
                    StatusCode::UNAUTHORIZED,
                    APIResponse::new(true, "No user found matching that handle").serialize()
            ));
        },
    };
    
    // destructure the user_entry object
    let (user, password_hash) = (user_entry.inner_user, user_entry.password_hash);

    if body.password != password_hash {
        return Err((
                StatusCode::UNAUTHORIZED,
                APIResponse::new(true, "invalid password").serialize()
        ))
    }

    // return a jwt
    let token = match super::token::create_token(user.clone(), None) {
        Err(e) => {
            warn!("error creating token: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, APIResponse::new(true, "Internal Server Error. Please report to an admin (probably via email...)").serialize()))
        },
        Ok(token) => token,
    };
    
    Ok(json!({"error": false, "token": token}).to_string())
}

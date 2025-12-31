use axum::{
    body::Body, http::{HeaderMap, Request, StatusCode}
};
use crate::authentication::{token::validate_token, user::User};


/// (fake) middleware for authenticating clients on the API
/// TODO: figure out how on earth the axum middleware api is *supposed* to work
pub async fn authenticate(headers: HeaderMap) -> Result<User, StatusCode> {
    let token = {
        let result = match headers.get("x-auth-token") {
            Some(token) => token.to_str(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        match result {
            Ok(token) => token,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        }
    };

    match validate_token(token.to_string()) {
        Ok(user_obj) => return Ok(user_obj),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
}

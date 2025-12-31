use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::extract::Extension;
use crate::authentication::token::validate_token;
use crate::authentication::user::User;

pub async fn authenticate(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("x-auth-token") {
        if let Ok(token) = auth_header.to_str() {
            match validate_token(token.to_string()) {
                Ok(user_obj) => {
                    let mut req = req; // why does this work :skull:
                    req.extensions_mut().insert(Arc::new(user_obj));
                    return Ok(next.run(req).await);
                },
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn retreieve_user_extension(Extension(user_obj): Extension<Arc<User>>) -> Arc<User> {
    return user_obj;
}

use chrono::Utc;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::authentication::{
    user,
};

const JWT_LIFE_MINUTES: i64 = 30;
static JWT_SECRET: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::var("JWT_SECRET")
        .expect("Unable to retrieve JWT_SECRET env variable. A quick fix is `JWT_SECRET=\">>your secret phrase here<<\" cargo run`")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub expiration: usize,
   pub issued_at: usize,
   pub subject: usize,
   pub password: String,
   pub user: user::User
}

pub fn create_token(user: user::User, password: String) -> Result<String, ()> {
    let now = Utc::now();
    let expiration_time = now
        .checked_add_signed(chrono::Duration::minutes(JWT_LIFE_MINUTES))
        .expect("Invalid Timestamp")
        .timestamp();

    let claims = Claims {
        expiration: expiration_time as usize,
        issued_at: now.timestamp() as usize,
        subject: user.uid,
        user: user,
        password: password
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
        .map_err(|_| {return ()})
    
}

pub fn validate_jwt(token: &str) -> Result<user::User, Box<dyn std::error::Error>> {
    todo!()
}

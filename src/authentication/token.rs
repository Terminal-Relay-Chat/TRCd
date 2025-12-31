use std::ops::Sub;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::authentication::{
    user,
};

const JWT_LIFE_MINUTES: i64 = 30;
const HASHING_ALGORITHM: Algorithm = Algorithm::HS512;

static JWT_SECRET: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::var("JWT_SECRET")
        .expect("Unable to retrieve JWT_SECRET env variable. A quick fix is `JWT_SECRET=\">>your secret phrase here<<\" cargo run`")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub exp: usize, // shorthand for expiration, gets auto validated 
   pub sub: usize, // shorthand for subject, this is just the standard I guess
   pub issued_at: usize,
   pub user: user::User
}

pub fn create_token(user: user::User) -> Result<String, ()> {
    let now = Utc::now();
    let expiration_time = now
        .checked_add_signed(chrono::Duration::minutes(JWT_LIFE_MINUTES))
        .expect("Invalid Timestamp")
        .timestamp();

    let claims = Claims {
        exp: expiration_time as usize,
        issued_at: now.timestamp() as usize,
        sub: user.uid,
        user: user,
    };

    let header = Header::new(HASHING_ALGORITHM);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
        .map_err(|_| {return ()})
    
}

pub fn validate_jwt(token: String) -> Result<user::User, Box<dyn std::error::Error>> {
    // exp (expiration appears to be auto validated)
    let result = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::new(HASHING_ALGORITHM))?;

    Ok(result.claims.user)
}

// tests
#[test]
fn test_create_valid_jwt() {
    use crate::authentication::user::{User, UserMode, UserPermissions};
    
    // create a dummy user to test on
    let dummy_user = User {
        user_type: UserMode::User,
        username: "dummy test user".to_string(),
        permission_level: UserPermissions::User,
        handle: "test_user".to_string(),
        provider_site: None,
        banned: false,
        uid: 0,
    };

    let result = create_token(dummy_user.clone()); // cloning because we need to validate it later
                                                   // on with the original result.
    
    // unwrap the result assuming it's ok
    assert!(result.is_ok(), "Expected no errors with a user this simple");
    let result = result.unwrap();
    
    let validation = validate_jwt(result);

    // unwrap the validation assuming it's ok
    assert!(validation.is_ok(), "Expected the JWT to be valid; this could trip if your computer takes 30 minutes to complete one test.");
    let final_user = validation.unwrap();

    assert_eq!(final_user, dummy_user, "Expected the JWT result to be the same as the initial User.");
}

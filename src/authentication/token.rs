use log::{warn}; 
use chrono::{DateTime, Utc};
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
   pub iat: usize, // needed for validation, shorthand for issued_at
   pub user: user::User
}

/// function to create a new Json Web Token from a User struct, pass None to the creation_time
/// argument (creation_time is used only for testing purposes and can cause security problems).
pub fn create_token(user: user::User, creation_time: Option<DateTime<Utc>>) -> Result<String, ()> {
    let now: DateTime<Utc> = match creation_time {
        Some(time) => { 
            warn!("create_token() called with a creation time, this is meant to only be used in testing and may cause unexpected problems!"); 
            time
        },
        None => Utc::now()
    };

    let expiration_time = now
        .checked_add_signed(chrono::Duration::minutes(JWT_LIFE_MINUTES))
        .expect("Invalid Timestamp")
        .timestamp();

    let claims = Claims {
        exp: expiration_time as usize,
        iat: now.timestamp() as usize,
        sub: user.uid,
        user: user,
    };

    let header = Header::new(HASHING_ALGORITHM);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
        .map_err(|_| {return ()})
    
}

/// Validate a jwt signed with the same JWT_SECRET as the one active.
pub fn validate_token(token: String) -> Result<user::User, Box<dyn std::error::Error>> {
    // exp (expiration appears to be auto validated)
    let result = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::new(HASHING_ALGORITHM))?;
    let user = result.claims.user; // make it owned
    
    Ok(user)
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

    let result = create_token(dummy_user.clone(), None); // cloning because we need to validate it later
                                                   // on with the original result.
    
    // unwrap the result assuming it's ok
    assert!(result.is_ok(), "Expected no errors with a user this simple");
    let result = result.unwrap();
    
    let validation = validate_token(result);

    // unwrap the validation assuming it's ok
    assert!(validation.is_ok(), "Expected the JWT to be valid; this could trip if your computer takes 30 minutes to complete one test.");
    let final_user = validation.unwrap();

    assert_eq!(final_user, dummy_user, "Expected the JWT result to be the same as the initial User.");
}

#[test]
fn test_invalid_jwt() {
    use crate::authentication::user::{User, UserMode, UserPermissions};
    // test to make sure that a strait up invalid jwt doesn't work at all
    assert!(validate_token("not a valid jwt".to_string()).is_err(), "Expected an invalid jwt to not work");
    
    /* test to make sure that an expired jwt doesn't work */

    let alg_leeway = Validation::new(HASHING_ALGORITHM).leeway;
    
    let expired_time = Utc::now()
        .checked_sub_signed(chrono::Duration::minutes(JWT_LIFE_MINUTES + (alg_leeway as i64))) // notice how it's sub not
                                                                                               // add, it's in the past. 
        .expect("Invalid Timestamp");

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

    let expired_token = create_token(dummy_user, Some(expired_time)); // a token created in a
                                                                      // simulated past, just
                                                                      // barely past the expiration
                                                                      // time
    assert!(expired_token.is_ok(), "Simulated tokens shouldn't crash.");
    let expired_token = expired_token.unwrap(); // unwrap the result

    assert!(validate_token(expired_token).is_err(), "an expired token should not pass validation")
}

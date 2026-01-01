//! methods to get and change values within the minimalist database 

use crate::authentication::user::{User};
use serde::{Serialize, Deserialize};

/// This differs from crate::authentication::user::User as it is mostly for passwords and secure
/// fields
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntry {
    password_hash: String,
    uid: usize,
    pub inner: User
}

const MINIMALIST_STORAGE_PATH: &'static str = "~/.local/share/TRCd/users.csv";



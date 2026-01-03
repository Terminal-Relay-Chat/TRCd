//! User struct and related components
use serde::{Serialize, Deserialize};


/// This enum is used to differentiate between users and (registered) bots.
/// Expect bot users as this chat app is *clearly* for nerds.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UserMode {
    User,
    Bot
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UserPermissions {
    User, // basic things: join channels, read/write to those channels
    Moderator, // `/kick` people, `/ban` people of lower ranks
    Admin, // highest permission. Assumed owner or extremely trusted member 
}

/// the publicly available information for a given user that should be stored in state
/// password is only used in the login process
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] // PartialEq for testing convinience.
                                                           // See token.rs tests if you change
                                                           // this.
pub struct User {
    pub user_type: UserMode,
    pub permission_level: UserPermissions,
    pub username: String,
    pub handle: String,
    pub provider_site: Option<String>, // this is so people can know how to DM them
    pub banned: bool, // for while the user is stored in memory
}
impl User {}

//! Traits and template for any database

use crate::authentication::user::User;

/// WARNING: this struct contains secure fields. Don't use in insecure contexts
pub struct UserDBEntry {
    pub password_hash: String,
    pub username: String,
    pub inner_user: User,
}

/// Basic calls for a given database, things like adding and checking users
pub trait DBCalls {
    fn fetch_user(&self, username: &str) -> impl Future<Output = Result<UserDBEntry, Box<dyn std::error::Error>>>;
    fn add_user(&self, new_user: UserDBEntry) -> impl Future<Output = Result<User, Box<dyn std::error::Error>>>;

    #[allow(dead_code)] //TODO
    fn ban_user(&self, username: &str) -> Result<User, &'static str>;

    /// method to set up a given database, the "proper" way to do this would be migrations, but 
    /// this is a more simple aproach.
    fn setup(&self) -> impl Future<Output = ()>; // because we can't use the async keyword we need
                                                 // to return a Future
}

#[allow(dead_code)] // this is because the following trait is a potential feature, but
                    // should still be listed in the api
/// Advanced calls for a given database, things like storing messages
pub trait AdvancedDBCalls {}

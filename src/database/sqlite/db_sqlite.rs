use std::{str::FromStr, time::Duration};

use super::super::database::DBCalls;
use sqlx::{ConnectOptions, Connection, Pool, Sqlite, SqliteConnection, sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Row};
use crate::database::database::UserDBEntry;
use crate::authentication::user::User;

pub const DB_DEFAULT_URL: &'static str = "sqlite://database/TRCd.db";


#[derive(Debug, Clone)]
pub struct DB_Sqlite {
    conn: Pool<Sqlite>
}
impl DBCalls for DB_Sqlite {
    fn add_user(&self, new_user: crate::database::database::UserDBEntry) -> Result<crate::authentication::user::User, &'static str> {
        todo!()
    }

    fn ban_user(&self, username: String) -> Result<crate::authentication::user::User, &'static str> {
        todo!()
    }

    async fn fetch_user(&self, username: String) -> Result<UserDBEntry, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            "SELECT * FROM user WHERE username = $1",
        )
            .bind(&username)
            .fetch_one(&self.conn)
            .await?;
        
        // extract the user from the entry
        let user_value: User;
        let user_raw: Option<String> = row.get("user_json");

        if let Some(inner_user) = user_raw {
            user_value = serde_json::from_str(&inner_user)?;
        } else {
            return Err("Bad Database Entry, Value \"user_json\" missing. Please contact an admin.".to_string().into())
        }
        
        // extract the password from the entry
        let password_hash: String = match row.get("password_hash") {
            Some(field) => field,
            None => return Err("Bad Database Entry, Value \"password_hash\" missing. Please contact an admin.".to_string().into()),
        };

        // combine elements to make a user DB entry
        let result = UserDBEntry { 
                password_hash: password_hash,
                username: username,
                inner_user: user_value 
        };

        Ok(result)
    }

    async fn setup(&self) {
        sqlx::query(
                "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY,
                    password_hash TEXT NOT NULL,
                    username: TEXT NOT NULL,
                    user_json TEXT NOT NULL
                )",
            )
            .execute(&self.conn)
            .await
            .unwrap(); // safe to call unwrap because we need this program to crash if the function fails
    }
}

impl DB_Sqlite {
    pub async fn new(db_url: &str) -> Self {
        // this function will call unwrap() a few times, this is safe because we want the app to
        // fail if anything here doesn't do what is expected.
        let conn = SqlitePoolOptions::new()
            .max_connections(20)
            .idle_timeout(Duration::from_secs(60))
            .acquire_timeout(Duration::from_secs(5))
            .connect_with(
                SqliteConnectOptions::from_str(db_url)
                    .unwrap()
                    .create_if_missing(true)
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete)
            )
            .await
            .unwrap();
        

        DB_Sqlite {  
            conn: conn
        }
    } 
}

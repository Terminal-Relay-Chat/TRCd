use tokio;
use log::{warn, info};
use tokio::sync::mpsc::unbounded_channel;

#[forbid(unsafe_code)]

mod backend;
mod authentication;
mod database;

use backend::socket_server; // backend server instantiated by socket becuse shared state

use crate::database::database::DBCalls; 
                                       

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {serve().await}
    // otherwise the bin is being run to manipulate entries
    
    new_user().await;
}

async fn new_user() {
    use crate::database::sqlite::db_sqlite::{DB_Sqlite, DB_DEFAULT_URL};
    use crate::database::database::UserDBEntry;
    use crate::authentication::user::{User, UserPermissions, UserMode};
    
    println!("Creating a new user, enter details below:");

    fn user_input(prompt: &str) -> String {
        let mut input = String::new();
        println!("{}", prompt);

        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        input.trim().to_string()
    }

    let connection = DB_Sqlite::new(DB_DEFAULT_URL).await;
    connection.setup().await;
    
    let username = user_input("Enter a dank handle (@`your_handle_here`) >");
    let password = bcrypt::hash(user_input("Enter a secure password >"), bcrypt::DEFAULT_COST).unwrap();
    let new_user = UserDBEntry {
        password_hash: password, //TODO: hash passwords for security
        username: username.clone(),
        inner_user: User {
            user_type: UserMode::User,
            handle: username,
            username: user_input("Enter a dank username >"),
            permission_level: UserPermissions::User,
            banned: false,
            provider_site: Some(user_input("Link a website? >"))
        }
    };

    dbg!(connection.add_user(new_user).await);
}

async fn serve() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    let socks = socket_server::SocketServer::new(3001);
    // bizzarly, I have to start the api from the socket server because of some shared state. It's
    // weird. (see create_app()). This is to share a broadcast tx instance to the API so that users
    // can send new messages in a more "secure" fassion.
    socks.run().await;
}

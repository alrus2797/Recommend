extern crate books_connection;
extern crate diesel;

use self::books_connection::*;


use connection_manager::ConnectionManager;

use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not setted");
    let controller: BookConnection = BookConnection::establish_connection(database_url);

    let user = controller.get_user_by_id(276725);
    let item = controller.get_item_by_id(1);
    
    match user {
        Some(u) => println!("User {:#?}", u.ratings),
        None => {}
    }

    match item{
        Some(i) => println!("Ratings {:#?}", i.title),
        None => {}
    }
    
}
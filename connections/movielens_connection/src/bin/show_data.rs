extern crate movielens_connection;
extern crate diesel;

use self::movielens_connection::*;
use self::schema::*;
use self::models::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;



use connection_manager::{ItemTrait, UserTrait, ConnectionManager};

use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not setted");
    let controller: MovieLensConnection = MovieLensConnection::establish_connection(database_url);

    let user = controller.get_user_by_id(1);
    let item = controller.get_item_by_id(0);


    match user {
        Some(u) => println!("User {:#?}", u.ratings),
        None => {}
    }

    match item{
        Some(i) => println!("Ratings {:#?}", i.title),
        None => {}
    }

    let connection = PgConnection::establish("postgres://postgres:root@localhost/movielensdb").expect(&format!("Error connecting to db"));

    let all_ratings = ratings::table.get_results::<Rating>(&connection).unwrap().shrink_to_fit();
    // println!("Size {}", all_ratings.len());
    
    
    
}
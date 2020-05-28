extern crate movie_connection;
extern crate diesel;

use self::movie_connection::*;


use connection_manager::ConnectionManager;

use dotenv::dotenv;
use std::env;


fn main() {
    // let conn = connect("postgres://postgres:root@localhost/moviesdb");
    // let results = movies::table
    //     .load::<Movie>(&conn)
    //     .expect("Error when reading movies");
    
    //     println!("Displaying {} movies", results.len());

    //     for movie in results {
    //         println!("{} - {}", movie.id, movie.title);
    //     }

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not setted");
    let controller: MovieConnection = MovieConnection::establish_connection(database_url);

    let user = controller.get_user_by_id(3);
    let item = controller.get_item_by_id(1);
    println!("{:#?}", item.title);
    
}
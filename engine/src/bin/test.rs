extern crate engine;

use movie_connection::*;
use books_connection::*;
use movielens_connection::*;

use connection_manager::ConnectionManager;

use self::engine::*;


fn main(){
    let booksdb    = "postgres://postgres:root@localhost/booksdb";
    let moviesdb   = "postgres://postgres:root@localhost/moviesdb";
    let movielensdb = "postgres://postgres:root@localhost/movielensdb";
    let controller = MovieLensConnection::establish_connection(movielensdb.to_string());

    let user1 = controller.get_user_by_id(278804);
    let user2 = controller.get_user_by_id(211);

    if let Some(u1) = user1 {
        if let Some(u2) = user2{
            let res = engine::jaccard_distance(&u1.ratings, &u2.ratings);
            println!("Distance is: {}", res);
        }
        else{
            println!("Second user not found");
        }
    }
    else{
        println!("First user not found");
    }


    // let user1 = &controller.get_user_by_name("Stephen".to_string())[0];
    // let user2 = &controller.get_user_by_name("Amy".to_string())[0];
    // let res = engine::jaccard_distance(&user1.ratings, &user2.ratings);
    // println!("Distance: {}", res); 


}
extern crate engine;

use movie_connection::*;
use books_connection::*;
use movielens_connection::*;

use connection_manager::ConnectionManager;

use self::engine::*;

use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()>{
    let booksdb    = "postgres://postgres:root@localhost/booksdb";
    let moviesdb   = "postgres://postgres:root@localhost/moviesdb";
    let movielensdb = "postgres://postgres:root@localhost/movielensdb";
    let controller = MovieConnection::establish_connection(moviesdb.to_string());

    // let user1 = controller.get_user_by_id(278804);
    // let user2 = controller.get_user_by_id(211);

    // if let Some(u1) = user1 {
    //     if let Some(u2) = user2{
    //         let res = engine::jaccard_distance(&u1.ratings, &u2.ratings);
    //         println!("Distance is: {}", res);
    //     }
    //     else{
    //         println!("Second user not found");
    //     }
    // }
    // else{
    //     println!("First user not found");
    // }

    let mut file = File::create("Matrix.csv")?;

    let mut matrix : Vec<Vec<f64>>= Vec::new();

    let items = controller.get_all_ratings();
    let items_average = controller.get_average_by_user();
    
    let mut temp_matrix: Vec<f64> = Vec::new();
    let mut distance : f64;
    // for (_, item1) in &items{
    //     for (_, item2) in &items{
    //         distance = engine::adjusted_cosine_simmilarity(&item1, &item2, &items_average);
    //         temp_matrix.push(distance);
    //         file.write(distance.to_string().as_bytes());
    //         file.write(b", ");
    //     }
    //     file.write(b"\n");
    //     matrix.push(temp_matrix.clone());
    //     temp_matrix.clear();
    // }
    let res = engine::adjusted_cosine_simmilarity(&items[&2], &items[&1], &items_average);
    println!("Distance: {}", res);

    // println!("Shape: ({}, {})", matrix.len(), matrix[0].len());
    // println!("{:#?}", items_average);

    Ok(())
    // println!("{:#?}", items);

    

    // let user1 = &controller.get_user_by_name("Stephen".to_string())[0];
    // let user2 = &controller.get_user_by_name("Amy".to_string())[0];
    // let res = engine::jaccard_distance(&user1.ratings, &user2.ratings);
    // println!("Distance: {}", res); 


}
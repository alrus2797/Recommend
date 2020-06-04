extern crate movielens_connection;
extern crate diesel;

use self::movielens_connection::*;


use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewMovie, NewUser, NewRating};

use csv;
use indicatif::ProgressIterator;

use std::collections::HashSet;

fn load_users(conn: &PgConnection){

    let mut users = Vec::new();

    println!("Loading book users...");
    for user_id in 1..=283228 {
        users.push(NewUser {id: user_id});
    }
    println!("Inserting users in database");
    for chunk in users.chunks(10000).progress(){
        create_users(conn, chunk);
    }   
}

fn load_movielens(conn: &PgConnection) -> HashSet<i32>{
    let mut content = csv::ReaderBuilder::new()
    .has_headers(true)
    .delimiter(b',')
    .from_path("./data1/movies.csv")
    .expect("Couldn't load from csv file");

    let movie_records: Vec<_> =  content.records().collect();
    let mut movies = Vec::new();

	let mut movie_hash = HashSet::new();

    println!("Loading books...");
    for movie_record in movie_records.iter() {
        if let Ok(record) = movie_record {
            let id: i32 = record[0].parse().unwrap();
            let title = String::from(&record[1]);
            let genres = String::from(&record[2]);
            movies.push(NewMovie {id, title, genres});
        }
    }

    // println!("{} -> {:#?}", "3257224281", book_hash.get("3257224281"));//813

    println!("Inserting books in database");
    for chunk in movies.chunks(10000).progress(){
        create_movies(conn, chunk, &mut movie_hash);
    }
    return movie_hash;
    
}

fn load_ratings(conn: &PgConnection, movies_idx: &HashSet<i32>){
    let mut content = csv::ReaderBuilder::new()
    .has_headers(true)
    .delimiter(b',')
    .from_path("./data1/ratings.csv")
    .expect("Couldn't load from csv file");

    let rating_records: Vec<_> =  content.records().collect();
    let mut ratings = Vec::new();

    println!("Loading Ratings...");
    for rating_record in rating_records.iter() {
        if let Ok(record) = rating_record {
            let user_id: i32 = record[0].parse().unwrap();
            let movie_id: i32 = record[1].parse().unwrap();
            let score: f64 = record[2].parse().unwrap();
            match movies_idx.get(&movie_id) {
				Some(_exist) => {
                    ratings.push(NewRating {
                        user_id, 
                        movie_id, 
                        score
                    });
                },
                None => {
                }
            }
        }
    }

    

    println!("Inserting ratings in database");
    for chunk in ratings.chunks(10000).progress(){
        create_ratings(conn, chunk);
    }
    
}


fn main(){
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not setted");
    let conn = connect(&database_url);

    // let conn = connect("postgres://postgres:root@localhost/booksdb");

    // let results = users::table
    //     .load::<User>(&conn)
    //     .expect("Error when reading movies");
    load_users(&conn);
	let movies_idx = load_movielens(&conn);
	// println!("{:#?}", books_idx);
    load_ratings(&conn, &movies_idx)



    
}
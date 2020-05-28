extern crate movie_connection;
extern crate diesel;

use self::movie_connection::*;


use dotenv::dotenv;
use std::env;

use csv;



fn main(){
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url not setted");
    let conn = connect(&database_url);

    let mut content = csv::ReaderBuilder::new()
    .has_headers(true)
    .from_path("./data/movies.csv")
    .expect("Couldn't load from csv file");

    let names = content.headers().unwrap().iter().skip(1);
    for name in names {
        create_user(&conn, name);
    }

    let names_len = content.headers().unwrap().len();
    
    let mut score: f64;

    for (row_idx, r) in content.records().enumerate() {
        if let Ok(record) = r {
            let iterator = record.iter().skip(1);
            create_movie(&conn, record.get(0).unwrap());
            for (col_idx, r) in iterator.enumerate() {
                if r != "" {
                    score = r.parse().unwrap();
                    create_rating(&conn, (row_idx + 1) as i32, (col_idx + 1) as i32, score);
                }
            }
        }
    }
    
    // let new_movie = NewMovie{
    //     title: String::from("Konosuba")
    // };

    // diesel::
    //     insert_into(movies::table)
    //     .values(new_movie)
    //     .execute(&conn)
    //     .expect("Error while saving new Movie");
}
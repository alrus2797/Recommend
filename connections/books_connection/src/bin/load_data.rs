extern crate books_connection;
extern crate diesel;

use self::books_connection::*;


use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewBook, NewUser, NewRating};

use csv;
use indicatif::ProgressIterator;

use std::collections::HashMap;

fn load_users(conn: &PgConnection){
    let mut content = csv::ReaderBuilder::new()
    .has_headers(false)
    .delimiter(b';')
    .from_path("./data/BX-Users.csv")
    .expect("Couldn't load from csv file");

    let user_records: Vec<_> =  content.records().collect();
    let mut users = Vec::new();

    println!("Loading book users...");
    for user_record in user_records.iter() {
        if let Ok(record) = user_record {
            let id: i32 = record[0].parse().unwrap();
            let address = String::from(&record[1]);
            let age: Option<i16> = if &record[2] == "\\N" {
                None
            }
            else{
                Some(record[2].parse().unwrap())
            };
            users.push(NewUser {id, address, age});
        }
    }

    println!("Inserting users in database");
    for chunk in users.chunks(10000).progress(){
        create_users(conn, chunk);
    }   
}

fn load_books(conn: &PgConnection) -> HashMap<String, i32>{
    let mut content = csv::ReaderBuilder::new()
    .has_headers(false)
    .delimiter(b';')
    .from_path("./data/BX-Books.csv")
    .expect("Couldn't load from csv file");

    let book_records: Vec<_> =  content.records().collect();
    let mut books = Vec::new();

	let mut book_hash = HashMap::new();

    println!("Loading books...");
    for book_record in book_records.iter() {
        if let Ok(record) = book_record {
            let book_uid = String::from(&record[0]);
            let title = String::from(&record[1]);
            let author = String::from(&record[2]);
            let year: i16 = record[3].parse().unwrap();
            
			// book_hash.insert(book_uid.clone(), (current_idx + 1) as i32);
			// current_idx += 1;
            books.push(NewBook {book_uid, title, author, year});
        }
    }

    // println!("{} -> {:#?}", "3257224281", book_hash.get("3257224281"));//813

    println!("Inserting books in database");
    for chunk in books.chunks(10000).progress(){
        create_books(conn, chunk, &mut book_hash);
    }
    return book_hash;
    
}

fn load_ratings(conn: &PgConnection, books_idx: &HashMap<String, i32>){
    let mut content = csv::ReaderBuilder::new()
    .has_headers(false)
    .delimiter(b';')
    .from_path("./data/BX-Book-Ratings.csv")
    .expect("Couldn't load from csv file");

    let rating_records: Vec<_> =  content.records().collect();
    let mut users = Vec::new();

    println!("Loading Ratings...");
    for rating_record in rating_records.iter() {
        if let Ok(record) = rating_record {
            let user_id: i32 = record[0].parse().unwrap();
            let book_uid: String = String::from(&record[1]);
            let score: f64 = record[2].parse().unwrap();
            match books_idx.get(&book_uid) {
				Some(book_id) => {
                    users.push(NewRating {
                        user_id     : user_id, 
                        book_id     : *book_id, 
                        score       : score
                    });
                },
                None => {
                }
            }
        }
    }

    

    println!("Inserting ratings in database");
    for chunk in users.chunks(10000).progress(){
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
	let books_idx = load_books(&conn);
	// println!("{:#?}", books_idx);
    load_ratings(&conn, &books_idx)



    
}
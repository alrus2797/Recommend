#[macro_use]

extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;
use common_macros::hash_map;
use connection_manager::{ItemTrait, UserTrait, ConnectionManager};

use self::schema::*;
use self::models::*;

#[derive(Debug)]
pub struct BookUser {
    id: i32,
    address: String,
    age: Option<i16>,
    pub ratings: HashMap<i32, f64>
}

impl UserTrait for BookUser{
    fn id(&self) -> i32{
        self.id
    }

    fn name(&self) -> String{
        self.address.clone()
    }

    fn ratings(&self) -> HashMap<i32, f64> {
        self.ratings.clone()
    }

    fn data(&self) -> HashMap<String, String>{
        let mut self_data = hash_map!{
            "address".into() => self.address.clone(),
        };
        if let Some(age) = self.age{
            self_data.insert("age".into(), age.to_string());
        }
        return self_data;
    }
}


impl BookUser{
    fn create_from_model(user: &User, ratings: HashMap<i32, f64>) -> BookUser{
        BookUser{
            id : user.id,
            address: user.address.clone(),
            age : user.age,
            ratings : ratings
        }
    }
}

impl ItemTrait for Book{
    fn id(&self) -> i32{
        self.id
    }
    fn name(&self) -> String{
        self.book_uid.clone()
    }
    fn data(&self) -> HashMap<String, String>{
        hash_map!{
            "book_uid".into() => self.book_uid.clone(),
            "title".into() => self.title.clone(),
            "author".into() => self.author.clone(),
            "year".into() => self.year.to_string()
        }
    }
}

pub fn create_users(conn: &PgConnection, users: &[NewUser]){
    diesel::
        insert_into(users::table)
        .values(users)
        .execute(conn)
        .expect("Error while saving new User");
}

pub fn create_books(conn: &PgConnection, books: &[NewBook], books_hm: &mut HashMap<String, i32>){
    let books = diesel::
        insert_into(books::table)
        .values(books)
        .get_results::<Book>(conn)
        .expect("Error while saving new Book");

    for book in books{
        books_hm.insert(book.book_uid, book.id);
    }
}


pub fn create_ratings(conn: &PgConnection, ratings: &[NewRating]){
    diesel::
        insert_into(ratings::table)
        .values(ratings)
        .execute(conn)
        .expect("Error while saving new Rating");
}


pub fn connect(url : &str) -> PgConnection {
    PgConnection::establish(url).expect(&format!("Error connecting to {}", url))
}

pub struct BookConnection {
    connection: PgConnection,
}


impl ConnectionManager<BookUser, Book> for BookConnection{
    fn establish_connection(url: String) -> BookConnection {
        let connection = PgConnection::establish(&url).expect(&format!("Error connecting to {}", url));
        BookConnection { connection } 
    }

    fn get_user_by_id(&self, id: i32) -> BookUser {
        let user_result = users::table
            .find(id)
            .first::<User>(&self.connection)
            .expect("Error when reading user");

        let ratings = Rating::belonging_to(&user_result)
            .load::<Rating>(&self.connection)
            .expect("Error when reading ratings belonging user");

        let mut ratings_hm = HashMap::new();

        for rating in ratings{
            ratings_hm.insert(rating.book_id, rating.score);
        }
        
        BookUser::create_from_model(&user_result, ratings_hm)
    }

    fn get_user_by_name(&self, address: String) -> Vec<BookUser> {
        let user_result = users::table
            .filter(users::address.eq(address))
            .load::<User>(&self.connection)
            .expect("Error when reading users");
        
        let ratings = Rating::belonging_to(&user_result)
            .load::<Rating>(&self.connection)
            .expect("Error when reading ratings")
            .grouped_by(&user_result);

        
        let mut users = Vec::new();

        for (idx, user) in user_result.iter().enumerate(){
            let mut ratings_hm = HashMap::new();
            for rating in &ratings[idx]{
                ratings_hm.insert(rating.book_id, rating.score);
            }
            users.push(BookUser::create_from_model(user, ratings_hm));
        }
        return users;

    }

    fn get_item_by_id(&self, id: i32) -> Book {
        books::table
        .find(id)
        .first::<Book>(&self.connection)
        .expect("Error when reading books")
    }

    fn get_item_by_name (&self, title: String) -> Vec<Book> {
        books::table
        .filter(books::title.eq(title))
        .load::<Book>(&self.connection)
        .expect("Error when reading movies")
    }

    
}
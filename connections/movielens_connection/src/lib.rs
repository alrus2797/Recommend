#[macro_use]

extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;
use std::collections::HashSet;
use connection_manager::{ItemTrait, UserTrait, ConnectionManager};

use self::schema::*;
use self::models::*;
#[derive(Debug)]
pub struct MovieLenUser {
    id: i32,
    pub ratings: HashMap<i32, f64>
}

impl UserTrait for MovieLenUser{
    fn id(&self) -> i32{
        self.id
    }

    fn name(&self) -> String{
        String::from("-")
    }

    fn ratings(&self) -> HashMap<i32, f64> {
        self.ratings.clone()
    }
}


impl MovieLenUser{
    fn create_from_model(user: &User, ratings: HashMap<i32, f64>) -> MovieLenUser{
        MovieLenUser{
            id : user.id,
            ratings : ratings
        }
    }
}

impl ItemTrait for Movie{
    fn id(&self) -> i32{
        self.id
    }
    fn name(&self) -> String{
        self.title.clone()
    }
}

pub fn create_users(conn: &PgConnection, users: &[NewUser]){
    diesel::
        insert_into(users::table)
        .values(users)
        .execute(conn)
        .expect("Error while saving new User");
}

pub fn create_movies(conn: &PgConnection, movies: &[NewMovie], movies_hm: &mut HashSet<i32>){
    let movies = diesel::
        insert_into(movies::table)
        .values(movies)
        .get_results::<Movie>(conn)
        .expect("Error while saving new Book");

    for movie in movies{
        movies_hm.insert(movie.id);
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

pub struct MovieLensConnection {
    connection: PgConnection,
}


impl ConnectionManager<MovieLenUser, Movie> for MovieLensConnection{
    fn establish_connection(url: String) -> MovieLensConnection {
        let connection = PgConnection::establish(&url).expect(&format!("Error connecting to {}", url));
        MovieLensConnection { connection } 
    }

    fn get_user_by_id(&self, id: i32) -> Option<MovieLenUser> {
        let user_result = users::table
            .find(id)
            .first::<User>(&self.connection)
            .optional()
            .unwrap();
        if let Some(res) = user_result{
            let ratings = Rating::belonging_to(&res)
            .load::<Rating>(&self.connection)
            .expect("Error when reading ratings belonging user");

            let mut ratings_hm = HashMap::new();

            for rating in ratings{
                ratings_hm.insert(rating.movie_id, rating.score);
            }
            
            Some(MovieLenUser::create_from_model(&res, ratings_hm))
        }
        else{
            return None;
        }

    }

    fn get_user_by_name(&self, _name: String) -> Vec<MovieLenUser> {
        Vec::<MovieLenUser>::new()
    }

    fn get_item_by_id(&self, id: i32) -> Option<Movie> {
        let movie = movies::table
            .find(id)
            .first::<Movie>(&self.connection)
            .optional()
            .unwrap();
        if let Some(res) = movie{
            Some(res)
        }
        else{
            None
        }
    }

    fn get_item_by_name (&self, _name: String) -> Vec<Movie> {
        Vec::<Movie>::new()
    }


    fn get_all_ratings(&self) -> HashMap<i32, HashMap<i32, f64>> {
        todo!();
    }

    fn get_average_by_user(&self) -> HashMap<i32, f64> {
        todo!();
    }
}
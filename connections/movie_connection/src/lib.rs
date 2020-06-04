#[macro_use]

extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;
use connection_manager::{ItemTrait, UserTrait, ConnectionManager};

use self::schema::*;
use self::models::*;
#[derive(Debug)]
pub struct MovieUser {
    id: i32,
    name: String,
    pub ratings: HashMap<i32, f64>
}

impl UserTrait for MovieUser{
    fn id(&self) -> i32{
        self.id
    }

    fn name(&self) -> String{
        self.name.clone()
    }

    fn ratings(&self) -> HashMap<i32, f64> {
        self.ratings.clone()
    }
}


impl MovieUser{
    fn create_from_model(user: &User, ratings: HashMap<i32, f64>) -> MovieUser{
        MovieUser{
            id : user.id,
            name : user.name.clone(),
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

pub fn create_user(conn: &PgConnection, c_name: &str){
    diesel::
        insert_into(users::table)
        .values(NewUser{name: String::from(c_name)})
        .execute(conn)
        .expect("Error while saving new User");
}

pub fn create_movie(conn: &PgConnection, c_title: &str){
    diesel::
        insert_into(movies::table)
        .values(NewMovie{title: String::from(c_title)})
        .execute(conn)
        .expect("Error while saving new Movie");
}


pub fn create_rating(conn: &PgConnection, c_movie_id: i32, c_user_id: i32, c_score: f64){
    diesel::
        insert_into(ratings::table)
        .values(NewRating{movie_id: c_movie_id, user_id: c_user_id, score: c_score})
        .execute(conn)
        .expect("Error while saving new Rating");
}


pub fn connect(url : &str) -> PgConnection {
    PgConnection::establish(url).expect(&format!("Error connecting to {}", url))
}

pub struct MovieConnection {
    connection: PgConnection,
}


impl ConnectionManager<MovieUser, Movie> for MovieConnection{
    fn establish_connection(url: String) -> MovieConnection {
        let connection = PgConnection::establish(&url).expect(&format!("Error connecting to {}", url));
        MovieConnection { connection } 
    }

    fn get_user_by_id(&self, id: i32) -> Option<MovieUser> {
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
            
            Some(MovieUser::create_from_model(&res, ratings_hm))
        }
        else{
            None
        }


    }

    fn get_user_by_name(&self, name: String) -> Vec<MovieUser> {
        let user_result = users::table
            .filter(users::name.eq(name))
            .load::<User>(&self.connection)
            .expect("Error when reading users");
        
        let ratings = Rating::belonging_to(&user_result)
            .load::<Rating>(&self.connection)
            .expect("Error when reading ratings")
            .grouped_by(&user_result);

        
        let mut movie_users = Vec::new();

        for (idx, user) in user_result.iter().enumerate(){
            let mut ratings_hm = HashMap::new();
            for rating in &ratings[idx]{
                ratings_hm.insert(rating.movie_id, rating.score);
            }
            movie_users.push(MovieUser::create_from_model(user, ratings_hm));
        }

        return movie_users;

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

    fn get_item_by_name (&self, name: String) -> Vec<Movie> {
        movies::table
        .filter(movies::title.eq(name))
        .load::<Movie>(&self.connection)
        .expect("Error when reading movies")
    }

    
}
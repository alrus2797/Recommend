use super::schema::*;
// Select Models
#[derive(Identifiable, Queryable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32
}

#[derive(Identifiable, Queryable, Debug)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub genres: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User)]
#[belongs_to(Movie)]
#[table_name = "ratings"]
pub struct Rating{
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub score: f64,
}

// Insert Models

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie {
    pub id: i32,
    pub title: String,
    pub genres: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: i32
}

#[derive(Insertable)]
#[table_name="ratings"]
pub struct NewRating {
    pub user_id: i32,
    pub movie_id: i32,
    pub score: f64,
}
use super::schema::*;
// Select Models
#[derive(Identifiable, Queryable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub address: String,
    pub age: Option<i16>,
}

#[derive(Identifiable, Queryable, Debug)]
pub struct Book {
    pub id: i32,
    pub book_uid: String,
    pub title: String,
    pub author: String,
    pub year: i16,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User)]
#[belongs_to(Book)]
#[table_name = "ratings"]
pub struct Rating{
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub score: f64,
}

// Insert Models

#[derive(Insertable, Debug)]
#[table_name="books"]
pub struct NewBook {
    pub book_uid: String,
    pub title: String,
    pub author: String,
    pub year: i16,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: i32,
    pub address: String,
    pub age: Option<i16>,
}

#[derive(Insertable)]
#[table_name="ratings"]
pub struct NewRating {
    pub user_id: i32,
    pub book_id: i32,
    pub score: f64,
}
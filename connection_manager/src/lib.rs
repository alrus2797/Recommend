use std::collections::HashMap;


pub trait ConnectionManager<User, Item>
{
    fn establish_connection(url: String)->Self;

    fn get_user_by_id(&self, id: i32) -> Option<User>;
    fn get_user_by_name(&self, name: String) -> Vec<User>;

    fn get_item_by_id(&self, id: i32) -> Option<Item>;
    fn get_item_by_name(&self, name: String) -> Vec<Item>;

    // fn get_all_users(&self) -> Vec<User>;
    fn get_all_ratings(&self) -> HashMap<i32, HashMap<i32, f64>>;
    fn get_average_by_user(&self) -> HashMap<i32, f64>;
}

pub trait UserTrait
{
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn ratings(&self) -> HashMap<i32, f64>;
    fn data(&self) -> HashMap<String, String>{
        HashMap::new()
    }
}

pub trait ItemTrait
{
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn data(&self) -> HashMap<String, String>{
        HashMap::new()
    }
}
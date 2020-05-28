use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait ConnectionManager<User, Item>
{
    fn establish_connection(url: String)->Self;

    fn get_user_by_id(&self, id: i32) -> User;
    fn get_user_by_name(&self, name: String) -> Vec<User>;

    fn get_item_by_id(&self, id: i32) -> Item;
    fn get_item_by_name(&self, name: String) -> Vec<Item>;
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
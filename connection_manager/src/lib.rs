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
    fn establish_connection(url: String);

    fn get_user_by_id(id: u64) -> User;
    fn get_user_by_name(name: String) -> User;

    fn get_item_by_id(id: u64) -> Item;
    fn get_item_by_name(name: String) -> Item;
}

pub trait User
{
    fn id(&self) -> u64;
    fn name(&self) -> String;
    fn ratings(&self) -> HashMap<u64, f64>;
    fn data(&self) -> HashMap<String, String>;
}

pub trait Item
{
    fn id(&self) -> u64;
    fn name(&self) -> String;
    fn data(&self) -> HashMap<String, String>;
}
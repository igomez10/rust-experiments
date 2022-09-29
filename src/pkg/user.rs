extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new() -> User {
        User {
            id: 1,
            name: "ignacio".to_string(),
            email: "someemail".to_string(),
            password: "password".to_string(),
        }
    }
}

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod pkg;
#[tokio::main]
async fn main() {
    println!("\n");

    println!("Hello, world!");

    let usera = User {
        id: 1,
        email: "someemail".to_string(),
        name: "ignacio".to_string(),
        password: "password".to_string(),
    };

    println!("{}", usera.name);
    let somedog = Dog {
        id: 1,
        name: "some".to_string(),
        age: 1,
        breed: "some".to_string(),
    };

    make_walk(somedog);
    make_walk(usera);
}

struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}

struct Dog {
    id: i32,
    name: String,
    age: i32,
    breed: String,
}
// walks a distance in a direction
trait Walker {
    fn walk(&self, distance: i32) -> i32;
}

trait Runner {
    fn run(&self, distance: i32) -> i32;
}

impl Walker for User {
    fn walk(&self, distance: i32) -> i32 {
        println!("im a user and i move {} units", distance);
        return 10;
    }
}

impl Walker for Dog {
    fn walk(&self, distance: i32) -> i32 {
        println!("im a dog and i move {} units", distance);
        return 10;
    }
}

impl Runner for User {
    fn run(&self, distance: i32) -> i32 {
        println!("im a user and i run {} units", distance);
        return 10;
    }
}

impl Runner for Dog {
    fn run(&self, distance: i32) -> i32 {
        println!("im a dog and i run {} units", distance);
        return 10;
    }
}

fn make_walk(walker: impl Walker) {
    walker.walk(10);
}

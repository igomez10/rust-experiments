extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::{io::Write, net::TcpListener};

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

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on port 7878");
    for stream in listener.incoming() {
        if stream.is_err() {
            println!("Error: {}", stream.unwrap_err());
            continue;
        }
        let mut stream = stream.unwrap();
        let status_line = "HTTP/1.1 200 OK";
        let contents = "Hello, World!";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

        println!("Connection established!");
        drop(stream)
    }
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

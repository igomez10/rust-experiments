#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use std::sync::Mutex;

struct SharedData {
    numbers: Mutex<Vec<i32>>,
}

#[get("/count")]
fn get_number(shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    // shared_data.numbers.lock().unwrap().push(number);
    format!(
        "There are now {} numbers",
        shared_data.numbers.lock().unwrap().len()
    )
}

#[get("/add")]
fn add_number(shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    shared_data.numbers.lock().unwrap().push(1);
    format!(
        "There are now {} numbers",
        shared_data.numbers.lock().unwrap().len()
    )
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_number, add_number])
        .manage(SharedData {
            numbers: Mutex::new(vec![5]),
        })
        .launch();
}

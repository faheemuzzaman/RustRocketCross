// #[macro_use] extern crate rocket_contrib;
// use rocket_contrib::json::JsonValue;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use rocket_contrib::{json::{Json}};

// #[derive(Serialize)]
// pub struct Student<'a, Storage: ?Sized> {
//   pub id: i64,
//   pub title: str
// }

// fn main(){
//     println!("{:?}",JsonTest());
// }

// fn JsonTest()-> JsonValue{
//     let mut data: Vec<Student> = Vec::new();
//     data.insert(01, "terms");
//     Json(data)
// }


#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// #[cfg(test)] mod tests;
use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}


// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![new])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
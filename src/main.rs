#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
extern crate rocket_cors;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Method; // 1.
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};
use rocket::State;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicUsize, Ordering};

type ID = usize;
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Message {
    id: ID,
    contents: String
}
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.      
        "http://localhost:8080/html/",
        "http://localhost:8080",
        "http://0.0.0.0:8080",        
        // "chrome-extension://fhbjgbiflinjbdggehcddcbncdddomop",               
    ]);

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/")]
fn hello() -> JsonValue {
    // println!("{:?}",&foo);
    json!([{
        "id": "01",
        "name": "Faheem"
        },
        {
            "id": "02",
            "name": "Faheem"
        },
        {
            "id": "02",
            "name": "Faheem"
        }]
    )  
}
type MessageMap = Mutex<HashMap<ID,String>>;

#[post("/add", data = "<user_input>")]
fn helloPost(user_input: Json<Message>, map: State<'_, MessageMap>) {
    println!("{:?}",user_input.0.contents);
}

fn main() {
    rocket().launch();
}

fn rocket()-> rocket::Rocket{
    rocket::ignite()
    .mount("/", routes![hello,helloPost]).attach(make_cors())
    .manage(Mutex::new(HashMap::<ID, String>::new()))
}




//Cargo.toml

// [dependencies]
// rocket = "^0.4.4"
// serde = "^1.0"
// serde_derive = "^1.0"
// serde_json = "^1.0"
// dotenv = "^0.14.1"
// reqwest = "^0.9.19"
// rocket_cors = "^0.5.0"
// lazy_static = "^1.4.0"



// [dependencies.rocket_contrib]
// version = "0.4.2"
// default-features = false
// features = ["tera_templates", "json"]







// Index.html

// <script src="https://code.jquery.com/jquery-3.4.1.js"
//     integrity="sha256-WpOohJOqMqqyKL9FccASB9O0KwACQJpFTUBLTYOVvVU="crossorigin="anonymous"></script>
// <link rel="stylesheet"
//     href="https://maxcdn.bootstrapcdn.com/bootstrap/3.4.0/css/bootstrap.min.css">
// <script src="https://code.jquery.com/jquery-3.4.1.js"
//     integrity="sha256-WpOohJOqMqqyKL9FccASB9O0KwACQJpFTUBLTYOVvVU="crossorigin="anonymous"></script>
// <link rel="stylesheet" type="text/css"
//     href="https://cdn.datatables.net/1.10.20/css/jquery.dataTables.css">

// <script type="text/javascript" charset="utf8"
//     src="https://cdn.datatables.net/1.10.20/js/jquery.dataTables.js"></script>

// <script>
// $(document).ready(function () {
//         BindTable();
//         $("#btnSend").click(function () {
//             $.ajax({
//                 url: "http://localhost:8000/add",
//                 type: "POST",   
//                 data: JSON.stringify({	
// 					"id": 01,
// 					"contents": $('#student').val() 
// 				}),				
//             });            
//         });
//     });

//     function BindTable()
// {
    
//     $.ajax({
//         type: "GET",
//         url: "http://localhost:8000/",                
//         success: function (msg) {            ;                        
//             $.each(msg, function (index) { 
//                 var row = '<tr><td> ' + msg[index].id + ' </td> <td> ' + msg[index].name + ' </td></tr>';                                                            
//                 $("#tbDetails").append(row);
// 				console.log(msg);
//             });             
//         }
//     });

// }

// </script>
// <label for="student">Add student</label>
// <input id="student" name="student" type="text" value="" />
// <input id="btnSend" type="button" value="Send" />


// <table id="tbDetails" border="1" cellpadding="2">
//     <tr>
//         <td>ID</td> <td>Name</td>
//     </tr>
//     <tbody>        
//     </tbody>
// </table>



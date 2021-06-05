//! Jonathan Rivera and Tram Vuong 2021
//! 
//! Sources:
//! https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
//! https://rocket.rs/v0.4/guide/
//! https://www.youtube.com/watch?v=2RWXeosWhAQ
//! https://www.shawntabrizi.com/code/combining-rocket-with-reqwest-to-call-an-api-with-rust/

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate reqwest;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::http::RawStr;
use reqwest::Error;
use reqwest::header;
use reqwest::blocking::Response;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use serde_json::Value as JsonValue;


// #[derive(Serialize, Deserialize, Debug)]
// struct Pokemon {
//     name: String,
//     id: u32,
//     image_url: String,
// }

// #[derive(FromForm)]
// struct Search{
//     query: String,
// }


#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "PokeSearch")]
        .iter().cloned().collect();

    Template::render("index", &context)
}

#[get("/search/<pokemon>")]
fn search(pokemon: String) -> Template {
    // let request_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    // let res = reqwest::blocking::get(request_url).json::<HashMap<String, String>>();
    println!("{:?}", pokemon);
    let base_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    let full_url = &base_url[..];
    let client = reqwest::blocking::Client::new();

    let mut response = client.get(full_url)
        .send()
        .unwrap(); 

    let data: JsonValue = response.json().unwrap();

    Template::render("search", &data)
}

// implement 404 handling

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, search])
        .attach(Template::fairing())
        .launch();
}
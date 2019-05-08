#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate serde_json;
use rocket::State;
use serde_json::{Value};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn quiz(seed: &str, data: &Value) -> Value {
    let category: &Value = &data["categories"]["generelt.txt"];
    let mut q: Value = serde_json::from_str("[]").unwrap();
    // let q: Value = category.clone();
    for item in category.as_array().unwrap() {
        let pair = item.as_array().unwrap();
        let question = Value::String(pair[0].to_string());
        let answer = Value::String(pair[1].to_string());
        let mut v = vec![question, answer];
        q.as_array_mut().unwrap().append(&mut v);
    }
    return q;
}

#[get("/")]
fn index(state: State<Value>) -> String {
    let q: Value = quiz("", &state["no"]);
    format!("Quiz: {}", serde_json::to_string_pretty(&q).unwrap())
}

fn init_quiz(path: &str) -> Result<Value, Box<Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let q = serde_json::from_reader(reader)?;
    Ok(q)
}

fn main() {
    let state : Value = init_quiz("output.json").unwrap();
    rocket::ignite()
        .manage(state)
        .mount("/", routes![index]).launch();
}

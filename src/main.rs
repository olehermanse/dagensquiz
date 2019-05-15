#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;
extern crate rand;
extern crate chrono;
extern crate crypto;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufRead};

use rocket::{State, Request};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use rand::Rng;
use chrono::{DateTime, Utc};

fn randint(seed: &str) -> usize {
    let mut hasher = Sha256::new();
    hasher.input_str(seed);
    let mut bytes: [u8; 32] = [0; 32];
    hasher.result(&mut bytes);
    let mut result : usize = 0;
    for i in 0..8 {
        result = result << 8;
        result += bytes[i] as usize;
    }
    return result;
}

fn randint_range(seed: &str, min: usize, max: usize) -> usize {
    assert!(min <= max);
    let r = randint(seed);
    let diff = max - min;
    let r = r % (diff + 1);
    let r = r + min;
    return r;
}

pub struct HostHeader<'a>(pub &'a str);

impl<'a, 'r> FromRequest<'a, 'r> for HostHeader<'a> {
    type Error = ();
    fn from_request(request: &'a Request) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h)),
            None => Outcome::Forward(()),
        }
    }
}

struct Question {
    question: String,
    answer: String,
}

struct Quiz {
    questions: Vec<Question>,
}

struct LocalizedQuizData {
    categories: HashMap<String, Quiz>,
}

struct QuizData {
    languages: HashMap<String, LocalizedQuizData>,
}

impl Quiz {
    fn read_file(path: &std::path::Path) -> Result<Quiz, Box<Error>> {
        let mut quiz : Quiz = Quiz { questions : vec![] };
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let line : String = String::from(line?);
            let v: Vec<&str> = line.split("  - ").collect();
            if v.len() == 2 {
                let q = String::from(v[0].trim());
                let a = String::from(v[1].trim());
                let question = Question {question: q, answer: a};
                quiz.questions.push(question);
            } else {
                println!("Skipping bad line: {}", line);
            }
        }
        return Ok(quiz);
    }
}

impl LocalizedQuizData {
    fn read_dir(path: &std::path::Path) -> Result<LocalizedQuizData, Box<Error>> {
        let mut data : LocalizedQuizData = LocalizedQuizData { categories : HashMap::new() };
        for entry in fs::read_dir(path)? {
            match entry {
                Ok(e) => {
                    let category_path = e.path();
                    match Quiz::read_file(&category_path) {
                        Ok(q) => {
                            let category = String::from(category_path.file_name().unwrap().to_str().unwrap());
                            data.categories.insert(category, q);
                        }
                        Err(_) => {
                            println!("Error while reading quiz {:?}", path);
                        }
                    }

                }
                Err(_) => {
                    println!("An error occured while reading {:?}", path);
                }
            }

        }
        return Ok(data);
    }
}

impl QuizData {
    fn add_language(&mut self, path: &std::path::Path) {
        let value = match LocalizedQuizData::read_dir(path){
            Ok(q) => {q}
            Err(_) => {return;}
        };
        let path = path.file_name().unwrap().to_str();
        let key = match path {
            Some(p) => p,
            None => {
                return;
            }
        };
        let key = String::from(key);
        println!("Adding language directory: {}", key);
        self.languages.insert(key, value);
    }
}

fn init_quiz(path: &str) -> Result<QuizData, Box<Error>> {
    let mut q: QuizData = QuizData {
        languages: HashMap::new(),
    };
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            q.add_language(&path);
        }
    }
    return Ok(q);
}

fn quiz(seed: &str, data: &QuizData, language: &str) -> Quiz {
    let mut counter = 0;
    let categories = &data.languages[language].categories;
    let mut quiz = Quiz {questions: vec![]};
    for _ in 0..2 {
        for category in categories {
            if quiz.questions.len() >= 10 {
                break;
            }
            let category = category.1;
            let tmp_seed = format!("{}_{}", seed, counter);
            counter += 1;
            let length = category.questions.len();
            let max = length - 1;
            let index = randint_range(&tmp_seed, 0, max);
            let question: String = category.questions[index].question.clone();
            let answer: String = category.questions[index].answer.clone();
            let question = Question { question: question, answer: answer };
            quiz.questions.push(question);
        }
    }
    let max = quiz.questions.len() - 1;
    for _ in 0..20 {
        let tmp_seed = format!("{}_{}", seed, counter);
        counter += 1;
        let index = randint_range(&tmp_seed, 1, max);
        quiz.questions.swap(0, index);
    }
    return quiz;
}

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    solution: &'static str,
    more: &'static str,
    date: String,
    url: &'static str,
    questions: Vec<String>,
    answers: Vec<String>,
}

fn templated_quiz(
    quiz: Quiz,
    seed: String,
    title: &'static str,
    solution: &'static str,
    more: &'static str,
    root: &'static str
) -> Template {
    let date = match seed.contains("-") {
        true => {seed}
        false => {format!("#{}", seed)}
    };
    let mut questions: Vec<String> = vec![];
    let mut answers: Vec<String> = vec![];
    for question in quiz.questions {
        questions.push(question.question);
        answers.push(question.answer);
    }
    let context = TemplateContext {
        title: title,
        solution: solution,
        more: more,
        date: date,
        url: root,
        questions: questions,
        answers: answers,
    };
    return Template::render("index", &context);
}

fn norwegian_template(seed: String, state: State<QuizData>, root: &'static str) -> Template {
    let quiz = quiz(&seed, &state, "no");
    return templated_quiz(quiz, seed, "Dagens quiz", "Fasit", "Mer quiz...", root);
}

fn english_template(seed: String, state: State<QuizData>, root: &'static str) -> Template {
    let quiz = quiz(&seed, &state, "en");
    return templated_quiz(quiz, seed, "Daily Quiz", "Solution", "More quiz...", root);
}

fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    return format!("{}", now.format("%Y-%m-%d"));
}

fn random_quiz_number() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(100, 999);
}

#[get("/")]
fn root() -> Redirect {
    return Redirect::to(format!("/{}", get_date()));
}

#[get("/random")]
fn random() -> Redirect {
    Redirect::to(format!("/{}", random_quiz_number()))
}

#[get("/no/random")]
fn norwegian_random() -> Redirect {
    Redirect::to(format!("/no/{}", random_quiz_number()))
}

#[get("/en/random")]
fn english_random() -> Redirect {
    Redirect::to(format!("/en/{}", random_quiz_number()))
}

#[get("/no/<seed>")]
fn norwegian_seed(seed: String, state: State<QuizData>) -> Template {
    return norwegian_template(seed, state, "/no/");
}

#[get("/no")]
fn norwegian() -> Redirect {
    return Redirect::to(format!("/no/{}", get_date()));
}

#[get("/en/<seed>")]
fn english_seed(seed: String, state: State<QuizData>) -> Template {
    return english_template(seed, state, "/en/");
}

#[get("/en")]
fn english() -> Redirect {
    return Redirect::to(format!("/en/{}", get_date()));
}

#[get("/<seed>")]
fn seed_host(seed: String, state: State<QuizData>, host: HostHeader) -> Template {
    if host.0.contains("dailyquiz.app") {
        return english_template(seed, state, "/");
    }
    return norwegian_template(seed, state, "/");
}

fn main() {
    let state: QuizData = init_quiz("2018").unwrap();
    rocket::ignite()
        .manage(state)
        .mount("/", routes![
            root,
            random,
            seed_host,
            norwegian,
            norwegian_seed,
            norwegian_random,
            english,
            english_seed,
            english_random])
        .attach(Template::fairing())
        .launch();
}

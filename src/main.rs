#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate crypto;
extern crate rand;
extern crate rocket_contrib;
extern crate serde_json;

use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::{Request, State};
use rocket_contrib::templates::Template;

use chrono::{DateTime, Utc};
use rand::Rng;

mod quiz;
mod randomish;
use quiz::{init_quiz, quiz, Quiz, QuizData};

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

#[derive(Serialize)]
struct TemplateContext<'a> {
    title: &'static str,
    solution: &'static str,
    more: &'static str,
    date: String,
    url: &'static str,
    questions: Vec<&'a str>,
    answers: Vec<&'a str>,
}

fn templated_quiz(
    quiz: Quiz,
    seed: String,
    title: &'static str,
    solution: &'static str,
    more: &'static str,
    root: &'static str,
) -> Template {
    let date = match seed.contains("-") {
        true => seed,
        false => format!("#{}", seed),
    };
    let mut questions: Vec<&str> = vec![];
    let mut answers: Vec<&str> = vec![];
    for question in &quiz.questions {
        questions.push(&question.question);
        answers.push(&question.answer);
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
    let state: QuizData = init_quiz("quiz").unwrap();
    rocket::ignite()
        .manage(state)
        .mount(
            "/",
            routes![
                root,
                random,
                seed_host,
                norwegian,
                norwegian_seed,
                norwegian_random,
                english,
                english_seed,
                english_random
            ],
        )
        .attach(Template::fairing())
        .launch();
}

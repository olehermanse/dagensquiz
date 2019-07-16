use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use crate::randomish::{randint_range, shuffle};

#[derive(Clone)]
pub struct Question {
    pub question: String,
    pub answer: String,
}

pub struct Quiz {
    pub questions: Vec<Question>,
}

struct LocalizedQuizData {
    categories: HashMap<String, Quiz>,
}

pub struct QuizData {
    languages: HashMap<String, LocalizedQuizData>,
}

fn split_line(line: &str) -> Vec<String> {
    let line: String = String::from(line);
    return line.split("  - ").map(String::from).collect();
}

fn read_lines(path: &std::path::Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut v: Vec<String> = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        v.push(line?);
    }
    return Ok(v);
}

impl Quiz {
    fn read_file(path: &std::path::Path) -> Result<Quiz, Box<dyn Error>> {
        let mut quiz: Quiz = Quiz { questions: vec![] };

        let v = read_lines(path)?;
        for line in v.into_iter() {
            let v = split_line(&line);
            if v.len() == 2 {
                let q = String::from(v[0].trim());
                let a = String::from(v[1].trim());
                let question = Question {
                    question: q,
                    answer: a,
                };
                quiz.questions.push(question);
            } else if line.trim() != "" {
                println!("Skipping bad line: {}", line);
            }
        }
        return Ok(quiz);
    }

    fn contains(&self, question: &Question) -> bool {
        for q in &self.questions {
            if q.question == question.question {
                return true;
            }
        }
        return false;
    }
}

impl LocalizedQuizData {
    fn read_dir(path: &std::path::Path) -> Result<LocalizedQuizData, Box<dyn Error>> {
        let mut data: LocalizedQuizData = LocalizedQuizData {
            categories: HashMap::new(),
        };
        for entry in fs::read_dir(path)? {
            match entry {
                Ok(e) => {
                    let category_path = e.path();
                    match Quiz::read_file(&category_path) {
                        Ok(q) => {
                            let category =
                                String::from(category_path.file_name().unwrap().to_str().unwrap());
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
        let value = match LocalizedQuizData::read_dir(path) {
            Ok(q) => q,
            Err(_) => {
                return;
            }
        };
        let path = path.file_name().unwrap().to_str();
        let key = match path {
            Some(p) => p,
            None => {
                return;
            }
        };
        let key = String::from(key);
        self.languages.insert(key, value);
    }
}

pub fn init_quiz(path: &str) -> Result<QuizData, Box<dyn Error>> {
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

pub fn quiz(seed: &str, data: &QuizData, language: &str) -> Quiz {
    let mut quiz = Quiz { questions: vec![] };

    let categories = &data.languages[language].categories;
    let mut keys: Vec<&String> = Vec::from_iter(categories.keys());
    keys.sort_unstable();
    shuffle(seed, &mut keys);

    let mut counter = 0;
    for _ in 0..2 {
        for key in &keys {
            if quiz.questions.len() >= 10 {
                break;
            }
            let category = &categories[*key];
            let tmp_seed = format!("{}_{}", seed, counter);
            counter += 1;
            let length = category.questions.len();
            let max = length - 1;
            let index = randint_range(&tmp_seed, 0, max);
            let question = &category.questions[index];
            if !quiz.contains(question) {
                quiz.questions.push(question.clone());
            }
        }
    }
    shuffle(seed, &mut quiz.questions);
    return quiz;
}

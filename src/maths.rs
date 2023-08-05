use rand::prelude::*;
use std::fmt;

#[derive(Clone)]
enum Operation {
    Addition,
}

#[derive(Clone)]
pub struct Question {
    a: i32,
    operation: Operation,
    b: i32,
    pub answer: i32,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}", self.a, self.b)
    }

}

impl Question {
    pub fn solution(&self) -> String {
        format!("{} = {}", self, self.answer)
    }
}

pub fn get_question() -> Question {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=100);
    let b = rng.gen_range(1..=100);
    let answer = a + b;
    Question { a, b, answer,
        operation: Operation::Addition
    }
}

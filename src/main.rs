extern crate chrono;

use std::io::{self, BufReader};

struct Requirement {
    name: String,
    explanation: String,
}

struct Specification {
    name: String,
    maintainer: String,
    requirements: Vec<Requirement>,
}

impl Requirement {
    fn new(name: &str, explanation: &str) -> Self {
        Requirement {
            name: name.to_string(),
            explanation: explanation.to_string(),
        }
    }
}

impl Specification {
    fn new(name: &str, maintainer: &str, requirements: Vec<Requirement>) -> Self {
        Specification {
            name: name.to_string(),
            maintainer: maintainer.to_string(),
            requirements: requirements
        }
    }
}

fn main() {
    println!("Hello, world!");
}

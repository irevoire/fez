use std::io::{self, BufRead, BufReader};

type Cube = Vec<char>;

struct A {
    cubes: Vec<Cube>,
}

impl A {
    pub fn new() -> Self {
        Self {
            cubes: vec![
                vec!['a', 'g', 'm', 's'],
                vec!['b', 'h', 'n', 't'],
                vec!['e', 'k', 'q', 'x', 'z'],
                vec!['a', 'g', 'm', 's'],
                vec!['b', 'h', 'n', 't'],
                vec!['c', 'i', 'o', 'u', 'v'],
                vec!['b', 'h', 'n', 't'],
                vec!['f', 'l', 'r', 'y'],
            ],
        }
    }

    pub fn check(&self, s: &str) -> bool {
        if s.len() != self.cubes.len() {
            return false;
        }
        let mut cubes: Vec<Cube> = self.cubes.clone();

        for c in s.chars() {
            if let Some(position) = cubes.iter().position(|cube| cube.contains(&c)) {
                cubes.remove(position);
            } else {
                return false;
            }
        }

        true
    }
}

fn main() {
    let a = A::new();

    let reader = BufReader::new(io::stdin());
    for line in reader.lines() {
        let line = line.unwrap();
        let word: String = line.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

        if a.check(&word) {
            println!("{}", line);
        }
    }
}

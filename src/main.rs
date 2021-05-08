use std::io::{self, BufRead, BufReader};

type Cube = Vec<char>;

struct A {
    cubes: Vec<Cube>,
}

impl A {
    pub fn new() -> Self {
        Self {
            cubes: vec![
                vec!['a', 'm', 'g', 's'],
                vec!['t', 'h', 'b', 'n'],
                vec!['e', 'z', 'k', 'x', 'q'],
                vec!['a', 'm', 'g', 's'],
                vec!['t', 'h', 'b', 'n'],
                vec!['v', 'u', 'c', 'o'],
                vec!['t', 'h', 'b', 'n'],
                vec!['r', 'f', 'y', 'l'],
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

        if a.check(&line) {
            println!("{}", line);
        }
    }
}

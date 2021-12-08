use std::{fs, path::Path};

fn main() {
    let answer = get_answer("day8a.txt");
    println!("Answer: {:#?}", answer);
}

fn get_answer<P: AsRef<Path>>(path: P) -> usize {
    let raw = fs::read_to_string(path).expect("failed to read file");
    let count = raw
        .lines()
        .map(|l| l.split('|').last().unwrap())
        .map(String::from)
        .map(|l| l.split(' ').map(String::from).collect::<Vec<String>>())
        .map(Display::new)
        .map(|d| d.count_all())
        .sum();
    count
}

#[derive(Debug)]
struct Display(Vec<String>);

impl Display {
    fn new(inner: Vec<String>) -> Self {
        Self(inner)
    }

    fn count_1(&self) -> usize {
        self.0.iter().filter(|c| c.len() == 2).count()
    }

    fn count_4(&self) -> usize {
        self.0.iter().filter(|c| c.len() == 4).count()
    }

    fn count_7(&self) -> usize {
        self.0.iter().filter(|c| c.len() == 3).count()
    }

    fn count_8(&self) -> usize {
        self.0.iter().filter(|c| c.len() == 7).count()
    }

    fn count_all(&self) -> usize {
        self.count_1() + self.count_4() + self.count_7() + self.count_8()
    }
}

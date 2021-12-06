use std::{fs, path::Path};

fn main() {
    let raw_input = get_raw("day6a.txt");
    println!("Raw input: {}\n", raw_input);

    let days = parse_raw(&raw_input);
    println!("Fishes: {:?}\n", days);

    let fishes: Vec<LanternFish> = days
        .into_iter()
        .map(|d| LanternFish::new(d, Stage::Adult))
        .collect();
    println!("Lantern fishes: {:?}\n", fishes);

    let mut system = EcoSystem::new(fishes);
    println!("Ecosystem of lantern fishes: {:?}\n", system);

    for _ in 0..80 {
        system.tick();
    }

    println!("Answer: {}", system.population());
}

fn get_raw<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Failed reading the file")
}

fn parse_raw(raw: &str) -> Vec<u32> {
    raw.split(',')
        .map(|token| token.parse::<u32>().expect("parsing number failed"))
        .collect()
}

#[derive(Debug, PartialEq)]
enum Stage {
    Young,
    Adult,
}

#[derive(Debug)]
struct LanternFish {
    days: u32,
    stage: Stage,
}

impl LanternFish {
    fn new(days: u32, stage: Stage) -> Self {
        Self { days, stage }
    }

    fn tick(&mut self) -> Option<LanternFish> {
        if self.days == 0 {
            self.days = 6;
        } else {
            self.days -= 1;
        }

        self.spawn()
    }

    fn spawn(&mut self) -> Option<LanternFish> {
        if self.days == 6 {
            if self.stage == Stage::Young {
                self.grows();
                None
            } else {
                Some(LanternFish::new(8, Stage::Young))
            }
        } else {
            None
        }
    }

    fn grows(&mut self) {
        self.stage = Stage::Adult;
    }
}

#[derive(Debug)]
struct EcoSystem(Vec<LanternFish>);

impl EcoSystem {
    fn new(fishes: Vec<LanternFish>) -> Self {
        Self(fishes)
    }

    fn tick(&mut self) {
        let mut new_borns = Vec::new();
        for f in &mut self.0 {
            let new_born = f.tick();
            if let Some(n) = new_born {
                new_borns.push(n);
            }
        }
        self.0.append(&mut new_borns)
    }

    fn population(&self) -> usize {
        self.0.len()
    }
}

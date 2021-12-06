use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    fs,
    path::Path,
};

fn main() {
    let raw_input = get_raw("day6a.txt");
    let mut system = parse_raw(&raw_input);

    for i in 0..=256 {
        let day = (i + 6) % 7;
        system.tick(day);
    }

    println!("Answer: {}", system.population());
}

fn get_raw<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Failed reading the file")
}

fn parse_raw(raw: &str) -> EcoSystem {
    let heap: BinaryHeap<u64> = raw
        .split(',')
        .map(|token| token.parse::<u64>().expect("parsing number failed"))
        .collect();
    let sorted: Vec<u64> = heap.into_sorted_vec();

    let mut adults = HashMap::new();
    for i in 0..7 {
        let pop = sorted.iter().filter(|&&n| n == i).count();
        adults.insert(i, FishesPop::new(pop));
    }

    EcoSystem::new(adults)
}

#[derive(Debug, PartialEq, Clone)]
enum Stage {
    Young,
    Adult,
}

#[derive(Debug, Clone)]
struct FishesPop {
    pop: usize,
}

impl FishesPop {
    fn new(pop: usize) -> Self {
        Self { pop }
    }

    fn spawn(&self) -> usize {
        self.pop
    }
}

#[derive(Debug)]
struct EcoSystem {
    adults: HashMap<u64, FishesPop>,
    youngs: VecDeque<FishesPop>,
}

impl EcoSystem {
    fn new(adults: HashMap<u64, FishesPop>) -> Self {
        Self {
            adults,
            youngs: VecDeque::new(),
        }
    }

    fn tick(&mut self, day: u64) {
        if let Some(selected_adults) = self.adults.get_mut(&day) {
            // youngs becomes adults
            if self.youngs.len() > 8 {
                let new_adults = self.youngs.pop_back().unwrap();
                selected_adults.pop += new_adults.pop;
            }

            // spanws new_borns
            let count = selected_adults.spawn();
            let young_pop = FishesPop::new(count);
            self.youngs.push_front(young_pop);
        } else {
            // youngs becomes adults
            if self.youngs.len() > 8 {
                let new_adults = self.youngs.pop_back().unwrap();
                self.adults.insert(day, new_adults);
            }

            // spawns zero pop of youngs
            let young_pop = FishesPop::new(0);
            self.youngs.push_front(young_pop);
        }
    }

    fn population(&self) -> usize {
        let mut total_pops = 0;
        for pop in self.adults.values() {
            total_pops += pop.pop;
        }

        for pop in &self.youngs {
            total_pops += pop.pop;
        }
        total_pops
    }
}

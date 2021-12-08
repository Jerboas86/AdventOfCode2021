use std::{fs, path::Path};

fn main() {
    let answer = get_answer("day8a.txt");
    println!("Answer: {:#?}", answer);
}

fn get_answer<P: AsRef<Path>>(path: P) -> usize {
    let raw = fs::read_to_string(path).expect("failed to read file");
    let mut displays = Vec::new();
    for line in raw.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let patterns: Vec<String> = parts[0].split(' ').map(|p| p.to_string()).collect();
        let display: Vec<String> = parts[1].trim().split(' ').map(|p| p.to_string()).collect();
        let d = Display::new(patterns, display);
        displays.push(d);
    }
    let mut total = 0;
    for d in &mut displays {
        total += d.count();
    }

    total
}

#[derive(Debug)]
struct Display {
    patterns: Vec<String>,
    display: Vec<String>,
    mapping: Vec<String>,
}

impl Display {
    fn new(patterns: Vec<String>, display: Vec<String>) -> Self {
        Self {
            patterns,
            display,
            mapping: vec!["".to_string(); 10],
        }
    }

    fn find_1(&mut self) {
        let pattern_1 = self
            .patterns
            .iter()
            .find(|c| c.len() == 2)
            .expect("No element maps to 1");
        self.mapping[1] = dbg!(pattern_1).to_owned();
    }

    fn find_3(&mut self) {
        let two_three_five: Vec<&String> = self.patterns.iter().filter(|p| p.len() == 5).collect();
        for pattern in two_three_five {
            let mut contained = 0;
            for c in self.mapping[1].chars() {
                if pattern.contains(c) {
                    contained += 1;
                }
            }
            if contained == self.mapping[1].len() {
                self.mapping[3] = pattern.to_owned();
            }
        }
    }

    fn find_4(&mut self) {
        let pattern_4 = self
            .patterns
            .iter()
            .find(|c| c.len() == 4)
            .expect("No element maps to 4");
        self.mapping[4] = pattern_4.to_owned();
    }

    fn find_2_and_5(&mut self) {
        let two_five: Vec<&String> = self
            .patterns
            .iter()
            .filter(|p| p.len() == 5)
            .filter(|&p| p != &self.mapping[3])
            .collect();
        for pattern in two_five {
            let mut count = 0;
            for c in self.mapping[6].chars() {
                if !pattern.contains(c) {
                    count += 1;
                }
            }
            if count == 1 {
                self.mapping[5] = pattern.to_owned();
            } else {
                self.mapping[2] = pattern.to_owned();
            }
        }
    }

    fn find_6(&mut self) {
        let zero_six_nine: Vec<String> = self
            .patterns
            .iter()
            .filter(|p| p.len() == 6)
            .map(|p| p.to_string())
            .collect();

        for pattern in zero_six_nine {
            let mut contained = 0;
            for c in self.mapping[1].chars() {
                if pattern.contains(c) {
                    contained += 1;
                }
            }

            if contained != self.mapping[1].len() {
                self.mapping[6] = pattern.to_owned();
            }
        }
    }

    fn find_7(&mut self) {
        let pattern_7 = self
            .patterns
            .iter()
            .find(|c| c.len() == 3)
            .expect("No element maps to 7");
        self.mapping[7] = pattern_7.to_owned();
    }

    fn find_8(&mut self) {
        let pattern_8 = self
            .patterns
            .iter()
            .find(|c| c.len() == 7)
            .expect("No element maps to 8");
        self.mapping[8] = pattern_8.to_owned();
    }

    fn find_0_and_9(&mut self) {
        let zero_nine: Vec<&String> = self
            .patterns
            .iter()
            .filter(|p| p.len() == 6)
            .filter(|&p| p != &self.mapping[6])
            .collect();

        for pattern in zero_nine {
            let mut contained = 0;
            for c in self.mapping[3].chars() {
                if pattern.contains(c) {
                    contained += 1;
                }
            }

            if contained == self.mapping[3].len() {
                self.mapping[9] = pattern.to_owned();
            } else {
                self.mapping[0] = pattern.to_string();
            }
        }
    }

    fn first_pass(&mut self) {
        self.find_1();
        self.find_4();
        self.find_7();
        self.find_8();
    }

    fn second_pass(&mut self) {
        self.find_6();
        self.find_3();
    }

    fn third_pass(&mut self) {
        self.find_0_and_9();
        self.find_2_and_5();
    }

    fn find_mapping(&mut self) {
        self.first_pass();
        self.second_pass();
        self.third_pass();
    }

    fn count(&mut self) -> usize {
        self.find_mapping();

        for p in &mut self.mapping {
            let mut p_chars: Vec<char> = p.chars().collect();
            p_chars.sort_unstable();
            *p = String::from_iter(p_chars);
        }

        for d in &mut self.display {
            let mut d_chars: Vec<char> = d.chars().collect();
            d_chars.sort_unstable();
            *d = String::from_iter(d_chars);
        }

        let mut nums = vec![0; 4];
        for (i, d) in self.display.iter().enumerate() {
            nums[i] = self.mapping.iter().position(|p| p == d).unwrap();
        }

        let strings: Vec<String> = nums.iter().map(|nb| nb.to_string()).collect();

        let num = strings.join("").parse::<usize>().unwrap();

        num
    }
}

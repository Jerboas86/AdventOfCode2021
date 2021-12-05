use std::fs;

fn main() {
    let raw_input = fs::read_to_string("day5a.txt").expect("failed to read input");

    let mut grid = Grid::new(1000, 1000);
    let mut lines = Vec::new();

    for line in raw_input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<u32> = points[0]
            .split(',')
            .map(|coords| coords.parse::<u32>().unwrap())
            .collect();
        let end: Vec<u32> = points[1]
            .split(',')
            .map(|coords| coords.parse::<u32>().unwrap())
            .collect();
        let line = Line::new(start, end);
        lines.push(line);
    }

    for line in &lines {
        grid.add(line);
    }

    println!("Answer: {:?}", grid.count());
}

struct Range {
    start: u32,
    end: u32,
    state: usize,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            state: 0,
        }
    }
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = None;
        if self.state <= (self.start as isize - self.end as isize).abs() as usize {
            match (self.start, self.end) {
                (s, e) if s < e => {
                    let v = s as usize + self.state;
                    item = Some(v);
                    self.state += 1;
                }
                (s, e) if s > e => {
                    let v = s as usize - self.state;
                    item = Some(v);
                    self.state += 1;
                }
                _ => {
                    let v = self.start as usize;
                    item = Some(v);
                }
            }
        }
        item
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(point: Vec<u32>) -> Self {
        Self {
            x: point[0],
            y: point[1],
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Vec<u32>, end: Vec<u32>) -> Self {
        let start = Point::new(start);
        let end = Point::new(end);
        Self { start, end }
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![0; width]; height],
        }
    }

    fn add(&mut self, line: &Line) {
        let start = &line.start;
        let end = &line.end;

        if line.is_horizontal() {
            for i in Range::new(start.y, end.y) {
                self.grid[i as usize][start.x as usize] += 1;
            }
        } else if line.is_vertical() {
            for i in Range::new(start.x, end.x) {
                self.grid[start.y as usize][i as usize] += 1;
            }
        } else {
            for (x, y) in Range::new(start.x, end.x).zip(Range::new(start.y, end.y)) {
                println!("Point({},{})", x, y);
                self.grid[y as usize][x as usize] += 1;
            }
        }
    }

    fn count(&self) -> u32 {
        let mut count = 0;
        for row in &self.grid {
            for &e in row {
                if e >= 2 {
                    count += 1;
                }
            }
        }
        count
    }
}

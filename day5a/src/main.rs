use std::fs;

fn main() {
    let raw_input = fs::read_to_string("day5a.txt").expect("failed to read input");

    let mut grid = Grid::new(1000, 1000);
    let mut lines = Vec::new();

    for line in raw_input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<i32> = points[0]
            .split(',')
            .map(|coords| coords.parse::<i32>().unwrap())
            .collect();
        let end: Vec<i32> = points[1]
            .split(',')
            .map(|coords| coords.parse::<i32>().unwrap())
            .collect();
        let line = Line::new(start, end);
        lines.push(line);
    }

    let straight_lines: Vec<Line> = lines.into_iter().filter(|l| l.is_straight()).collect();

    for line in &straight_lines {
        grid.add(line);
    }

    println!("Answer: {:?}", grid.count());
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(point: Vec<i32>) -> Self {
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
    fn new(start: Vec<i32>, end: Vec<i32>) -> Self {
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

    fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<i32>>,
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
            for i in start.y.min(end.y)..=end.y.max(start.y) {
                self.grid[i as usize][start.x as usize] += 1;
            }
            return;
        }

        if line.is_vertical() {
            for i in start.x.min(end.x)..=end.x.max(start.x) {
                self.grid[start.y as usize][i as usize] += 1;
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

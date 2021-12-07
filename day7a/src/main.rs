use std::{collections::BinaryHeap, fs, path::Path};

fn main() {
    let positions = get_positions("day7a.txt");
    let answer = find_least_dist(positions);
    println!("{}", answer);
}

fn get_positions<P: AsRef<Path>>(path: P) -> Vec<i32> {
    let raw = fs::read_to_string(path).expect("Failed reading file");
    let output: BinaryHeap<i32> = raw.split(',').map(|tk| tk.parse().unwrap()).collect();
    output.into_sorted_vec()
}

fn mean(positions: &[i32]) -> i32 {
    positions.iter().sum::<i32>() / positions.len() as i32
}

fn find_least_dist(positions: Vec<i32>) -> i32 {
    let mut ref_val = 100;
    let mut new_val = sum_diff(&positions, ref_val);
    while new_val.0 != ref_val {
        ref_val = new_val.0;
        new_val = sum_diff(&positions, new_val.0);
    }
    new_val.1
}

fn sum_diff(values: &[i32], ref_value: i32) -> (i32, i32) {
    let low_sum = values.iter().fold(0, |mut acc, &curr| {
        acc += (curr - (ref_value - 1)).abs();
        acc
    });
    let ref_sum = values.iter().fold(0, |mut acc, &curr| {
        acc += (curr - ref_value).abs();
        acc
    });
    let high_sum = values.iter().fold(0, |mut acc, &curr| {
        acc += (curr - (ref_value + 1)).abs();
        acc
    });

    let mut values = [
        (ref_value - 1, low_sum),
        (ref_value, ref_sum),
        (ref_value + 1, high_sum),
    ];

    values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    if values[0].1 > values[1].1 && values[2].1 > values[1].1 {
        (ref_value, ref_sum)
    } else {
        (values[0].0, values[0].1)
    }
}

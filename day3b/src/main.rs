use std::fs;

fn main() {
    let raw_input = fs::read_to_string("day3A.txt").expect("failed to read input");
    let mut input = Vec::new();

    for line in raw_input.lines() {
        let parsed = i16::from_str_radix(line, 2).expect("Parsing failed");
        input.push(parsed);
    }

    let oxygene_rate = get_oxygene(&input);
    let dioxyde_rate = get_dioxyde(&input);

    println!("oxygene rate: {}", oxygene_rate);
    println!("dioxyde rate: {}", dioxyde_rate);
    println!("The answer is: {}", oxygene_rate * dioxyde_rate);
}

fn get_bit(num: i16, position: i16) -> bool {
    (num & 1 << (11 - position)) != 0
}

fn get_bits(input: &[i16], position: i16) -> Vec<bool> {
    let mut bits = Vec::new();
    for &num in input {
        let bit = get_bit(num, position);
        bits.push(bit);
    }
    bits
}

fn get_most_common_bit(bits: &[bool]) -> bool {
    let ones_count = bits.iter().filter(|&&b| b).count();
    let zeros_count = bits.len() - ones_count;
    ones_count >= zeros_count
}

fn get_least_common_bit(bits: &[bool]) -> bool {
    let ones_count = bits.iter().filter(|&&b| b).count();
    let zeros_count = bits.len() - ones_count;
    ones_count < zeros_count
}

fn get_oxygene(input: &[i16]) -> i32 {
    let mut input = Vec::from(input);
    for pos in 0..=11 {
        let bits = get_bits(&input, pos);
        let most_common = get_most_common_bit(&bits);

        let filtered_input: Vec<i16> = bits
            .iter()
            .enumerate()
            .filter_map(|(idx, &b)| {
                if b == most_common {
                    Some(input[idx])
                } else {
                    None
                }
            })
            .collect();

        if filtered_input.len() == 1 {
            return filtered_input[0] as i32;
        } else {
            input = filtered_input;
        }
    }
    panic!("Oxygene filtering process failed")
}

fn get_dioxyde(input: &[i16]) -> i32 {
    let mut input = Vec::from(input);
    for pos in 0..=11 {
        let bits = get_bits(&input, pos);
        let least_common = get_least_common_bit(&bits);

        let filtered_input: Vec<i16> = bits
            .iter()
            .enumerate()
            .filter_map(|(idx, &b)| {
                if b == least_common {
                    Some(input[idx])
                } else {
                    None
                }
            })
            .collect();

        if filtered_input.len() == 1 {
            return filtered_input[0] as i32;
        } else {
            input = filtered_input;
        }
    }
    panic!("Dioxyde filtering process failed")
}

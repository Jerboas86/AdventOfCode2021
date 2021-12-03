use std::fs;

fn main() {
    let raw_input = fs::read_to_string("day3A.txt").expect("failed to read input");
    let mut input = Vec::new();

    for line in raw_input.lines() {
        let parsed = i16::from_str_radix(line, 2).expect("Parsing failed");
        input.push(parsed);
    }

    let gamma_rate = get_gamma_rate(&input);
    let epsilon_rate = get_epsilon_rate(&input);

    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("The answer is: {}", gamma_rate * epsilon_rate);
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

fn get_most_common_bit(bits: Vec<bool>) -> bool {
    let ones_count = bits.iter().filter(|&&b| b).count();
    let zeros_count = bits.len() - ones_count;
    ones_count > zeros_count
}

fn get_least_common_bit(bits: Vec<bool>) -> bool {
    let ones_count = bits.iter().filter(|&&b| b).count();
    let zeros_count = bits.len() - ones_count;
    ones_count < zeros_count
}

fn get_gamma_rate(input: &[i16]) -> i32 {
    let mut gamma_rate = String::new();
    for pos in 0..=11 {
        let bits = get_bits(input, pos);
        let most_common = get_most_common_bit(bits);
        if most_common {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0')
        }
    }
    i32::from_str_radix(&gamma_rate, 2).expect("Gamma parsing failed")
}

fn get_epsilon_rate(input: &[i16]) -> i32 {
    let mut epsilon_rate = String::new();
    for pos in 0..=11 {
        let bits = get_bits(input, pos);
        let least_common = get_least_common_bit(bits);
        if least_common {
            epsilon_rate.push('1');
        } else {
            epsilon_rate.push('0')
        }
    }
    i32::from_str_radix(&epsilon_rate, 2).expect("Gamma parsing failed")
}

use std::{fs, io};
use std::fs::File;
use std::io::BufRead;

fn debug_line_check(line: &str, remaining: &Vec<char>, matched: bool) {
    println!(
        "Checking line: '{}', Remaining: {:?}, Matched: {}",
        line, remaining, matched
    );
}

fn can_match(remaining: &[char], base_patterns: &[Vec<char>], memo: &mut Vec<Option<bool>>) -> bool {
    if remaining.is_empty() {
        return true; // Fully matched
    }

    let remaining_len = remaining.len();
    if let Some(result) = memo[remaining_len] {
        return result; // Use memoized result if available
    }

    for pattern in base_patterns {
        if remaining.starts_with(pattern) {
            if can_match(&remaining[pattern.len()..], base_patterns, memo) {
                memo[remaining_len] = Some(true);
                return true;
            }
        }
    }

    memo[remaining_len] = Some(false);
    false
}

pub fn find_possible_towel_patterns() -> i16 {
    let mut number_of_possible_towel_patterns = 0;

    let input_1 = fs::read_to_string("src/inputs/day19_q1_base_patterns").expect("missing input file");
    let base_patterns: Vec<Vec<char>> = input_1.lines().map(|l| l.chars().collect()).collect();

    if let Ok(file) = File::open("src/inputs/day19_q1_desired_patterns") {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(line) = line {
                let remaining = line.chars().collect::<Vec<_>>();
                let mut memo = vec![None; remaining.len() + 1]; // Memoization table

                if can_match(&remaining, &base_patterns, &mut memo) {
                    number_of_possible_towel_patterns += 1;
                    debug_line_check(&line, &remaining, true);
                } else {
                    debug_line_check(&line, &remaining, false);
                }
            }
        }
    }
    println!("Number of possible towel patterns: {}", number_of_possible_towel_patterns);
    number_of_possible_towel_patterns
}
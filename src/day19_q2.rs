use std::{fs, io};
use std::fs::File;
use std::io::BufRead;

fn debug_line_check(line: &str, remaining: &Vec<char>, total_ways: u64) {
    println!(
        "Checking line: '{}', Remaining: {:?}, Total Ways: {}",
        line, remaining, total_ways
    );
}

fn count_ways(remaining: &[char], base_patterns: &[Vec<char>], memo: &mut Vec<Option<u64>>) -> u64 {
    if remaining.is_empty() {
        return 1; // Fully matched, count as 1 way
    }

    let remaining_len = remaining.len();
    if let Some(result) = memo[remaining_len] {
        return result; // Use memoized result if available
    }

    let mut total_ways = 0;

    for pattern in base_patterns {
        if remaining.starts_with(pattern) {
            total_ways += count_ways(&remaining[pattern.len()..], base_patterns, memo);
        }
    }

    memo[remaining_len] = Some(total_ways);
    total_ways
}

pub fn find_total_towel_patterns() -> u64 {
    let mut total_patterns_ways = 0;

    let input_1 = fs::read_to_string("src/inputs/day19_q1_base_patterns").expect("missing input file");
    let base_patterns: Vec<Vec<char>> = input_1.lines().map(|l| l.chars().collect()).collect();

    if let Ok(file) = File::open("src/inputs/day19_q1_desired_patterns") {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(line) = line {
                let remaining = line.chars().collect::<Vec<_>>();
                let mut memo = vec![None; remaining.len() + 1]; // Memoization table
                let ways = count_ways(&remaining, &base_patterns, &mut memo);
                debug_line_check(&line, &remaining, ways);
                total_patterns_ways += ways;
            }
        }
    }
    println!("Total possible towel pattern combinations: {}", total_patterns_ways);
    total_patterns_ways
}
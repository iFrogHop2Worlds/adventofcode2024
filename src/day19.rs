use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::num::ParseIntError;

pub fn find_possible_towel_patterns() -> i32 {
    let mut number_of_possible_towel_patterns = 0;

    // We want to loop through all desired patterns and check if it is possible or not to make the
    // pattern using base patterns.
    // For each possible pattern increment number_of_possible_towel_patterns
    let input_1 = fs::read_to_string("src/inputs/day19_q1_base_patterns").expect("missing input file");
    let mut base_patterns: Vec<Vec<char>> = input_1.lines().map(|l| l.chars().collect()).collect();
    if let Ok(file) = File::open("src/inputs/day19_q1_base_patterns") {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(line) = line {
                let mut remaining = line.chars().collect::<Vec<_>>();
                let mut is_possible = true;

                while !remaining.is_empty() {
                    let mut matched = false;

                    for pattern in &base_patterns {
                        // Check if the start of remaining matches the current base pattern
                        if remaining.starts_with(pattern) {
                            // Remove the matched pattern from the remaining characters
                            remaining.drain(..pattern.len());
                            matched = true;
                            break;
                        }
                    }

                    // If no pattern matches, it means we can't build the line
                    if !matched {
                        is_possible = false;
                        break;
                    }
                }

                // Increment the counter if the line is possible
                if is_possible {
                    number_of_possible_towel_patterns += 1;
                }
            }
        }
    }
    println!("Number of possible towel patterns: {}", number_of_possible_towel_patterns);
    return number_of_possible_towel_patterns;
}
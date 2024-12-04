use regex::Regex;
use std::fs;
use std::error::Error;

pub fn scan_corrupted_mul_instructions(optimized: bool) -> Result<(), Box<dyn Error>> {
    let corrupted_memory = fs::read_to_string("src/inputs/day3_q1")?;
    let mut total_sum = 0;
    let mut summing_enabled = true;
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let dont_pattern = Regex::new(r"\s*don't\(\)\s*")?;
    let do_pattern = Regex::new(r"\s*do\(\)\s*")?;
    
    if optimized {
        let mut capture_position = 0;
        while capture_position < corrupted_memory.len() {
            let mut dnt_idx = capture_position;
            let mut do_idx = capture_position;
            let mut mul_idx = capture_position;
            if let Some(dont_capture) = dont_pattern.find(&corrupted_memory[capture_position..]) {
                dnt_idx = dont_capture.end();
                println!("Found dont pattern {}", dnt_idx);
            }
            if let Some(do_capture) = do_pattern.find(&corrupted_memory[capture_position..]) {
                do_idx = do_capture.end();
                println!("Found do pattern {}", do_idx);
            }
            if let Some(mul_capture) = mul_pattern.find(&corrupted_memory[capture_position..]) {
                mul_idx = mul_capture.end();
                println!("Found mul pattern {}", mul_idx);
            }
            
            if summing_enabled {
                if(mul_idx < dnt_idx) {
                    if let Some(mul_capture) = mul_pattern.captures(&corrupted_memory[capture_position..]) {
                        if let (Ok(x), Ok(y)) = (mul_capture[1].parse::<i64>(), mul_capture[2].parse::<i64>()) {
                            total_sum += x * y;
                        } else {
                            println!("Failed to parse captured numbers");
                        }
                        capture_position += mul_capture.get(0).map_or(0, |m| m.end());
                    }
                }
            } 
            if(dnt_idx < mul_idx) {
                if let Some(dont_capture) = dont_pattern.find(&corrupted_memory[capture_position..]) {
                    println!("Found don't pattern");
                    summing_enabled = false;
                    capture_position += dont_capture.end();
                }
            }
            if(do_idx < dnt_idx) {
                if let Some(do_capture) = do_pattern.find(&corrupted_memory[capture_position..]) {
                    println!("Found do pattern");
                    summing_enabled = true;
                    capture_position += do_capture.end();
                }
            } else {
                println!("No pattern found");
                capture_position += 1; 
            }
        }
    } else {
        for capture in mul_pattern.captures_iter(&corrupted_memory) {
            if let (Ok(x), Ok(y)) = (capture[1].parse::<i64>(), capture[2].parse::<i64>()) {
                total_sum += x * y;
            } else {
                println!("Failed to parse captured numbers");
            }
        }
    }

    println!("Total sum of all 'mul' sequences: {}", total_sum);
    Ok(())
}
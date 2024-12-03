use regex::Regex;

pub fn scan_corrupted_mul_instructions(optimized: bool) {
    let corrupted_memory = std::fs::read_to_string("src/inputs/day3_q1").expect("Failed to read file");

    let mut total_sum = 0;
    let mut summing_enabled = true;

    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    println!("mull pattern: {}", &mul_pattern );
    println!("dont pattern: {}", &dont_pattern );
    println!("do pattern: {}", &do_pattern);
    if optimized {
        let mut capture_position = 0;
        println!("current position 1: {}", &capture_position);
        // Iterate over the whole text once
        while capture_position < corrupted_memory.len() {
            println!("Current Position: {}", capture_position);
            println!("Summing Enabled: {}", summing_enabled);

            if summing_enabled {
                if let Some(mul_capture) = mul_pattern.captures(&corrupted_memory[capture_position..]) {
                    let x: i64 = mul_capture[1].parse().unwrap();
                    let y: i64 = mul_capture[2].parse().unwrap();
                    total_sum += x * y;
                    println!("Match found: x={}, y={}, Total Sum: {}", x, y, total_sum);
                    capture_position = corrupted_memory[capture_position..].find(&mul_capture[0]).unwrap() + mul_capture[0].len();
                    continue;
                }
            }
            println!("Evaluating slice at position {}: {:?}", capture_position, &corrupted_memory[capture_position..]);

            if let Some(dont_capture) = dont_pattern.find(&corrupted_memory[capture_position..]) {
                summing_enabled = false;
                println!("Disabling summing at position: {}", capture_position);
                capture_position = corrupted_memory[capture_position..].find(dont_capture.as_str()).unwrap() + dont_capture.end();
            } else if let Some(do_capture) = do_pattern.find(&corrupted_memory[capture_position..]) {
                summing_enabled = true;
                println!("Enabling summing at position: {}", capture_position);
                capture_position = corrupted_memory[capture_position..].find(do_capture.as_str()).unwrap() + do_capture.end();
            } else {
                println!("No match found at position: {}", capture_position);
                capture_position += 1; // Advance one character
            }
        }
    } else {
        for capture in mul_pattern.captures_iter(&corrupted_memory) {
            let x: i64 = capture[1].parse().unwrap();
            let y: i64 = capture[2].parse().unwrap();
            total_sum += x * y;
        }
    }

    println!("Total sum of all 'mul' sequences: {}", total_sum);
}
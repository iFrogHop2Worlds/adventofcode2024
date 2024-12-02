pub fn analyze_reactor_safety_reports() {
    let mut safe_count = 0;
    let unusual_data = std::fs::read_to_string("src/inputs/day2_q1").expect("Failed to read file");
    let lines = unusual_data.lines();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if numbers.len() < 2 {
            continue;
        }

        let mut is_consistent = true;
        let mut increasing = numbers[1] > numbers[0];

        for window in numbers.windows(2) {
            let diff = window[1] - window[0];

            if diff.abs() < 1 || diff.abs() > 3 {
                is_consistent = false;
                break;
            }

            if increasing && diff < 0 {
                is_consistent = false;
                break;
            }

            if !increasing && diff > 0 {
                is_consistent = false;
                break;
            }
        }

        if is_consistent {
            safe_count += 1;
        }
    }

    println!("The number of safe reports is: {}", safe_count);
}
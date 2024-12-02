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

        if is_consistent_sequence(&numbers) {
            safe_count += 1;
            continue;
        }

        let mut removable = false;
        for i in 0..numbers.len() {
            if is_consistent_sequence(
                &numbers
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &n)| if j != i { Some(n) } else { None })
                    .collect::<Vec<i32>>(),
            ) {
                removable = true;
                break;
            }
        }

        if removable {
            safe_count += 1;
        }
    }

    println!("The number of safe reports is: {}", safe_count);
}

fn is_consistent_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let mut increasing = numbers[1] > numbers[0];

    for window in numbers.windows(2) {
        let diff = window[1] - window[0];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if increasing && diff < 0 {
            return false;
        }

        if !increasing && diff > 0 {
            return false;
        }
    }

    true
}
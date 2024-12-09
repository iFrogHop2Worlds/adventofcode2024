use std::fs;

pub fn sum_of_valid_equations() -> i64 {
    let input = fs::read_to_string("src/inputs/day7_q1").expect("missing input file");
    input.lines().filter_map(|line| {
        let mut parts = line.split(": ");
        let target: i64 = parts.next()?.parse().ok()?;
        let numbers: Vec<i64> = parts.next()?.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        if find_valid_equation(&numbers, target) {
            Some(target)
        } else {
            None
        }
    }).sum()
}

fn find_valid_equation(numbers: &[i64], target: i64) -> bool {
    let n = numbers.len();
    if n == 0 {
        return false;
    }

    // 3^(n-1) combinations of operators (+, *, ||),
    // using two bits to represent each operator
    let operators_permutations = 1 << ((n - 1) * 2);

    for perm in 0..operators_permutations {
        let mut result = numbers[0];
        let mut current_perm = perm;

        for i in 0..n - 1 {
            let operator = current_perm & 0b11; // extract two bits for the current operator
            current_perm >>= 2;

            result = match operator {
                0b00 => result + numbers[i + 1],  // +
                0b01 => result * numbers[i + 1],  // *
                0b10 => {
                    // || (concatenation)
                    let left = result.to_string();
                    let right = numbers[i + 1].to_string();
                    let concatenated = left + &right;
                    concatenated.parse::<i64>().unwrap_or(0) 
                }
                _ => result, 
            };
        }

        if result == target {
            return true;
        }
    }
    false
}
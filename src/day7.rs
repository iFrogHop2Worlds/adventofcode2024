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
    let operators_permutations = 1 << (n - 1); // 2^(n-1) combinations of operators

    for perm in 0..operators_permutations {
        let mut result = numbers[0];
        for i in 0..n - 1 {
            if (perm & (1 << i)) != 0 {
                result += numbers[i + 1];
            } else {
                result *= numbers[i + 1];
            }
        }

        if result == target {
            return true;
        }
    }

    false
}
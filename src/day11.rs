
fn split_number(mut n: i64) -> (i64, i64) {
    let digits: Vec<_> = n.to_string().chars().collect();
    let len = digits.len();
    let mid = len / 2;
    let left = digits[..mid].iter().collect::<String>().parse::<i64>().unwrap_or(0);
    let right = digits[mid..].iter().collect::<String>().parse::<i64>().unwrap_or(0);
    (left, right)
}

pub fn evolve_stones(stones: &mut Vec<i64>) {

    let mut new_stones = Vec::new();

    for &stone in stones.iter() {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let num_digits = stone.to_string().len();
            if num_digits % 2 == 0 {
                let (left, right) = split_number(stone);
                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }

    stones.clear(); // Clear the old stones
    stones.extend(new_stones); // Extend the vector with new values
    println!("{:?}", stones.len());
}



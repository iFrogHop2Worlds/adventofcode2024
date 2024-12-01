use std::io::{self, Read};
pub fn process_file()  {
    let content = std::fs::read_to_string("src/inputs/day1_q1").expect("Failed to read file");

    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for line in content.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i64>(), numbers[1].parse::<i64>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }
    
    list1.sort();
    list2.sort();
    
    let sum_of_differences: i64 = list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs()) 
        .sum(); 

    println!(
        "The sum of the differences between corresponding elements is: {}",
        sum_of_differences
    );
    
}

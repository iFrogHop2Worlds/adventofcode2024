use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::num::ParseIntError;

fn parse_rules(data: &str) -> Result<Vec<(i32, i32)>, ParseIntError> {
    data.lines()
        .take(1176)
        .map(|line| {
            let parts: Vec<&str> = line.trim().split('|').collect();
            let before = parts[0].trim().parse::<i32>()?;
            let after = parts[1].trim().parse::<i32>()?;
            Ok((before, after))
        })
        .collect()
}

fn parse_updates(data: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    data.lines()
        .skip(1178)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim().split(',')
                .map(|num| num.trim().parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect()
}

fn reorder_update(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut ordered_update = update.to_vec();
    ordered_update.sort_by_key(|&num| {
        rules.iter().position(|&(before, after)| num == before || num == after)
    });
    ordered_update
}

pub fn fix_safteymanual_updates() {
    let input = fs::read_to_string("src/inputs/day5_q1").expect("missing input file");
    let rules_input = parse_rules(&input).expect("failed to parse rules");
    let updates = parse_updates(&input).expect("failed to parse updates");
    
    println!("Rules:");
    for rule in &rules_input {
        println!("{:?}", rule);
    }

    println!("\nUpdates:");
    for update in &updates {
        println!("{:?}", update);
    }
    
    let mut precedence: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (before, after) in rules_input {
        precedence.entry(after).or_default().insert(before);
    }
 
    fn is_correct_order(update: &Vec<i32>, precedence: &HashMap<i32, HashSet<i32>>) -> bool {
        // Map each page to its "actual" position in the update
        let mut position_map = HashMap::new();
        for (i, &page) in update.iter().enumerate() {
            position_map.insert(page, i);
        }
        
        for &page in update {
            if let Some(before_pages) = precedence.get(&page) {
                for &before_page in before_pages {
                    if let Some(&before_index) = position_map.get(&before_page) {
                        if before_index > position_map[&page] {
                            return false; 
                        }
                    }
                }
            }
        }
        true
    }
    
    let mut sum_of_middle_pages = 0;
    for update in updates {
        if is_correct_order(&update, &precedence) {
            let middle_index = update.len() / 2;
            sum_of_middle_pages += update[middle_index];
        }
    }

    println!("Sum of middle page numbers of correctly ordered updates: {}", sum_of_middle_pages);
}

pub fn incorrect_order_count() {
    let input = fs::read_to_string("src/inputs/day5_q1").expect("missing input file");
    let rules_input = parse_rules(&input).expect("failed to parse rules");
    let updates_input = parse_updates(&input).expect("failed to parse updates");

    let mut precedence: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (before, after) in &rules_input {
        precedence.entry(*after).or_default().insert(*before);
    }

    fn is_correct_order(update: &Vec<i32>, precedence: &HashMap<i32, HashSet<i32>>) -> bool {
        let mut position_map = HashMap::new();
        for (i, &page) in update.iter().enumerate() {
            position_map.insert(page, i);
        }

        for &page in update {
            if let Some(before_pages) = precedence.get(&page) {
                for &before_page in before_pages {
                    if let Some(&before_index) = position_map.get(&before_page) {
                        if before_index > position_map[&page] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    let mut middle_sum = 0;
    for update in updates_input {
        let ordered_update = reorder_update(&update, &rules_input);
        if update != ordered_update {
            let middle_index = ordered_update.len() / 2;
            middle_sum += ordered_update[middle_index];
        }
    }

    println!("The sum of the middle page numbers on incorrect updates is: {}", middle_sum);
}

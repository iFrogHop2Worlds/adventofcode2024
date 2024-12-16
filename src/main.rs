use aoc24::*;

fn main() {
    // // day1
    // process_input_and_calculate_differences();
    // process_input_and_calculate_similarities_score();
    // // day 2
    // analyze_reactor_safety_reports();
    //day 3
    // let _ = scan_corrupted_mul_instructions(true);
    // let _ = scan_corrupted_mul_instructions(false);
    //day 4
    //day4::solve_word_puzzle();
    //day4::solve_puzzle_mas_x();
    //day5
    //fix_safteymanual_updates();
    //incorrect_order_count();
    //day6
    //count_distinct_positions(89, 51);
    //day7
    //println!("sum of valid equations: {}", sum_of_valid_equations());
    //day8
    //println!("Number of unique antinode locations: {}", calculate_antinode_positions());
    //day9
    // let checksum = free_disk_space();
    // println!("The resulting filesystem checksum is: {}", checksum);
    //day10
    // let score = find_trailhead_scores::<i64>();
    // println!("Total trailhead score: {}", score);
    //day11
    let blinks = 25;
    let mut stones = vec![5, 89749, 6061, 43, 867, 1965860, 0, 206250];
    for _ in 0..blinks {
        evolve_stones(&mut stones);
    }
    println!("Stones: {:?}", stones);
    println!("Number of stones after 25 blinks: {}", &stones.len());
    
}

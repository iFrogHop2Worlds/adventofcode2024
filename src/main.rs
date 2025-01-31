use itertools::Itertools;
use aoc24::*;

fn main() {
    // day1
    // process_input_and_calculate_differences();
    // process_input_and_calculate_similarities_score();

    // day 2
    // analyze_reactor_safety_reports();

    // day 3
    // let _ = scan_corrupted_mul_instructions(true);
    // let _ = scan_corrupted_mul_instructions(false);

    // day 4
    //day4::solve_word_puzzle();
    //day4::solve_puzzle_mas_x();

    // day5
    //fix_safteymanual_updates();
    //incorrect_order_count();

    // day6
    //count_distinct_positions(89, 51);

    // day7
    //println!("sum of valid equations: {}", sum_of_valid_equations());

    // day8
    //println!("Number of unique antinode locations: {}", calculate_antinode_positions());

    // day9
    // let checksum = free_disk_space();
    // println!("The resulting filesystem checksum is: {}", checksum);

    // day10
    // let score = find_trailhead_scores::<i64>();
    // println!("Total trailhead score: {}", score);

    // day11
    // let blinks = 75;
    // let mut stones = vec![5, 89749, 6061, 43, 867, 1965860, 0, 206250];
    // for _ in 0..blinks {
    //     evolve_stones(&mut stones);
    // }
    // //println!("Stones: {:?}", stones);
    // println!("Number of stones after 75 blinks: {}", &stones.len());

    // day12
    // let total_cost = calculate_fencing_cost();
    // println!("Total cost for fencing the garden regions: {}", total_cost);

    // day13
    // let (prizes_won, total_tokens_spent) = solve_claw_machines();
    //
    // println!("Prizes won: {}", prizes_won);
    // println!("Total tokens spent: {}", total_tokens_spent);

    // day 14
    // predict_robots_movement();

    // day15
    // let initial_grid: Vec<Vec<char>> = std::fs::read_to_string("src/inputs/day15_q1")
    //     .expect("Failed to read file")
    //     .lines()
    //     .map(|line| line.chars().collect())
    //     .collect();
    //
    // let moves = std::fs::read_to_string("src/inputs/day15_q1moves").expect("Failed to read file"); //"<^^>>>vv<v>>v<<";
    //
    // println!("Moves: {}", moves);
    // let mut warehouse = Warehouse::new(initial_grid);
    // warehouse.print_grid();
    // let mut i = 0;
    // for mv in moves.chars() {
    //      i+=1;
    //      println!("Move {:?}, {}", i, mv);
    //     warehouse.step(mv);
    //     warehouse.print_grid();
    // }
    // //warehouse.print_grid();
    // let sum = warehouse.sum_gps_coordinates();
    // println!("Sum of all boxes' GPS coordinates: {}", sum);

    // day16
    // if let Some(score) = find_lowest_score() {
    //     println!("Lowest score: {}", score);
    // } else {
    //     println!("No valid path found.");
    // }

    // day17
    //three_bit_computer();

    // day 18
    //find_safe_memory_path();
    //question 2
    // match find_blocking_coordinate() {
    //     Some((x, y)) => println!("Blocking coordinate found: ({}, {})", x, y),
    //     None => println!("No single coordinate blocked the path."),
    // }

    // day 19
    find_possible_towel_patterns();
}

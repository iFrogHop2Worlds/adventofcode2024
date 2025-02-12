#![feature(int_roundings)]
pub mod day1;
pub use day1::process_input_and_calculate_differences;
pub use day1::process_input_and_calculate_similarities_score;

pub mod day2;
pub use day2::analyze_reactor_safety_reports;

pub mod day3;
pub use day3::scan_corrupted_mul_instructions;

pub mod day4;
pub use day4::solve_word_puzzle;
pub use day4::solve_puzzle_mas_x;

pub mod day5;
pub use day5::fix_safteymanual_updates;
pub use day5::incorrect_order_count;

pub mod day6;
pub use day6::count_distinct_positions;
pub mod day7;
pub use day7::sum_of_valid_equations;

pub mod day8;
//pub use day8::calculate_antinode_positions;

pub mod day9;
pub use day9::free_disk_space;

pub mod day10;
pub use day10::find_trailhead_scores;

pub mod day11;
pub use day11::evolve_stones;

pub mod day12;
pub use day12::calculate_fencing_cost;

pub mod day13;
pub use day13::solve_claw_machines;

pub mod day14;
pub use day14::predict_robots_movement;

pub mod day15;
pub use day15::Warehouse;

pub mod day16;
pub use day16::find_lowest_score;

pub mod day17;
pub use day17::three_bit_computer;

pub mod day18;
pub use day18::find_safe_memory_path;

pub mod day18_q2;
pub use day18_q2::find_blocking_coordinate;

pub mod day19;
pub use day19::find_possible_towel_patterns;
pub mod day19_q2;
pub use day19_q2::find_total_towel_patterns;

pub mod day20;
pub use day20::race_condition_simulator;


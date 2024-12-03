use aoc24::{
    process_input_and_calculate_differences, 
    process_input_and_calculate_similarities_score,
    analyze_reactor_safety_reports,
    scan_corrupted_mul_instructions
};
fn main() { 
    // // day1
    // process_input_and_calculate_differences();
    // process_input_and_calculate_similarities_score();
    // // day 2
    // analyze_reactor_safety_reports();
    
    //day 3
    scan_corrupted_mul_instructions(false);
    scan_corrupted_mul_instructions(true);
}

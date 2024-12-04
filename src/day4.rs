use std::fs;

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),  // Right
    (0, -1), // Left
    (1, 0),  // Down
    (-1, 0), // Up
    (1, 1),  // Down-right diagonal
    (1, -1), // Down-left diagonal
    (-1, 1), // Up-right diagonal
    (-1, -1) // Up-left diagonal
];

pub fn solve_word_puzzle() {
    let word_puzzle = fs::read_to_string("src/inputs/day4_q1").expect("missing input file");
    let grid: Vec<Vec<char>> = word_puzzle.lines()
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS";
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            for direction in &DIRECTIONS {
                if check_word(&grid, word, row as isize, col as isize, *direction) {
                    count += 1;
                }
            }
        }
    }

    println!("Total occurrences of {}: {}", word, count);
}

fn check_word(grid: &[Vec<char>], word: &str, start_row: isize, start_col: isize, direction: (isize, isize)) -> bool {
    let (row_dir, col_dir) = direction;
    let bytes = word.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        let new_row = start_row + i as isize * row_dir;
        let new_col = start_col + i as isize * col_dir;

        if new_row < 0 || new_col < 0 || new_row >= grid.len() as isize || new_col >= grid[0].len() as isize {
            return false;
        }
        if grid[new_row as usize][new_col as usize] != b as char {
            return false;
        }
    }
    true
}
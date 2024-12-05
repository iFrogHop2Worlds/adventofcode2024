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

pub fn solve_puzzle_mas_x() {
    let puzzle_input = fs::read_to_string("src/inputs/day4_q1").expect("missing input file");
    let grid: Vec<Vec<char>> = puzzle_input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let diagonal_directions = [
        (1, 1),  // Down-right
        (-1, -1), // Up-left
        (1, -1),  // Down-left
        (-1, 1)   // Up-right
    ];

    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'A' {
                for direction_index in 0..diagonal_directions.len() {
                    let direction_m = diagonal_directions[direction_index];
                    let direction_s = diagonal_directions[(direction_index + 2) % 4]; // Opposite direction for 'S'

                    if check_mas_any_direction(&grid, row as isize, col as isize, direction_m, direction_s) {
                        count += 1;
                    }

                    // Swap roles of M and S directions to cover opposite arrangement
                    if check_mas_any_direction(&grid, row as isize, col as isize, direction_s, direction_m) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Total diagonal X formations of MAS: {}", count);
}

fn check_mas_any_direction(
    grid: &[Vec<char>], row: isize, col: isize,
    direction_m: (isize, isize), direction_s: (isize, isize)
) -> bool {
    let (m_row_dir, m_col_dir) = direction_m;
    let (s_row_dir, s_col_dir) = direction_s;

    let new_m_row = row + m_row_dir;
    let new_m_col = col + m_col_dir;
    let new_s_row = row + s_row_dir;
    let new_s_col = col + s_col_dir;

    // Check boundaries and chars
    if new_m_row >= 0 && new_m_col >= 0 && new_s_row >= 0 && new_s_col >= 0 &&
        new_m_row < grid.len() as isize && new_m_col < grid[0].len() as isize &&
        new_s_row < grid.len() as isize && new_s_col < grid[0].len() as isize &&
        grid[new_m_row as usize][new_m_col as usize] == 'M' &&
        grid[new_s_row as usize][new_s_col as usize] == 'S'
    {
        return true;
    }

    false
}
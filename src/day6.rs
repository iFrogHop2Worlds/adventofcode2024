use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn count_distinct_positions(start_row: usize, start_col: usize) -> usize {
    // Test input
    // let mut grid = vec![
    //     vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
    //     vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
    // ];
    
    let input = fs::read_to_string("src/inputs/day6_q1")
        .expect("missing input file");

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut visited = HashSet::new();
    let mut dir = Direction::Up;
    let (mut row, mut col) = (start_row, start_col);
    let rows = grid.len();
    let cols = grid[0].len();

    visited.insert((row, col));
    grid[row][col] = 'x';

    loop {
    
        print_grid(&grid, row, col);

        let (next_row, next_col) = match dir {
            Direction::Up => (row.wrapping_sub(1), col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.wrapping_sub(1)),
        };

        if next_row >= rows || next_col >= cols {
            break;
        }

        if grid[next_row][next_col] == '#' {
            dir = dir.turn_right();
        } else {
            row = next_row;
            col = next_col;
            visited.insert((row, col));
            grid[row][col] = 'x';  // Mark visited place
        }
    }
    println!("Distinct positions visited: {}", &visited.len());
    visited.len()
}


fn print_grid(grid: &[Vec<char>], current_row: usize, current_col: usize) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if r == current_row && c == current_col {
                print!("X "); 
            } else {
                print!("{} ", cell);
            }
        }
        println!();
    }
    println!("---");
}
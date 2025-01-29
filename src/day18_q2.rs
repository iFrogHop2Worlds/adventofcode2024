use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

pub fn find_blocking_coordinate() -> Option<(usize, usize)> {
    const WIDTH: usize = 71;
    const HEIGHT: usize = 71;
    const FILENAME: &str = "src/inputs/day18_q1";

    // Create a grid with default value '.'
    let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];

    // Read all coordinates from the file
    let mut coordinates = Vec::new();
    if let Ok(file) = File::open(FILENAME) {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(coord) = line {
                let parts: Vec<&str> = coord.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                        if x < WIDTH && y < HEIGHT {
                            coordinates.push((x, y));
                        }
                    }
                }
            }
        }
    }

    // Process the coordinates one by one and run BFS
    for &(x, y) in &coordinates {
        // Mark the coordinate on the grid
        grid[y][x] = '#';

        // Run BFS from (0, 0) to (70, 70)
        if !is_path_possible(&grid, WIDTH, HEIGHT) {
            // If no path exists, return this blocking coordinate
            return Some((x, y));
        }
    }

    // If no coordinate blocks the path completely, return None
    None
}

// Function to check if a path exists from (0, 0) to (70, 70) using BFS
fn is_path_possible(grid: &Vec<Vec<char>>, width: usize, height: usize) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; width]; height];

    // Start BFS from (0, 0)
    queue.push_back((0, 0));
    visited[0][0] = true;

    // Directions for moving up, down, left, or right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y)) = queue.pop_front() {
        // If we've reached the target, return true
        if x == width - 1 && y == height - 1 {
            return true;
        }

        // Check all possible directions
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let (nx, ny) = (nx as usize, ny as usize);

                if !visited[ny][nx] && grid[ny][nx] != '#' {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // No valid path exists
    false
}

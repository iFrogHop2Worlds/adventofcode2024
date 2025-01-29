use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

pub fn find_safe_memory_path() -> usize {
    const WIDTH: usize = 71;
    const HEIGHT: usize = 71;
    const FILENAME: &str = "src/inputs/day18_q1";

    // Create a grid with default value '.'
    let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];

    // Read the file and update the grid with '#'
    if let Ok(file) = File::open(FILENAME) {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(coord) = line {
                let parts: Vec<&str> = coord.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                        if x < WIDTH && y < HEIGHT {
                            grid[y][x] = '#';
                        }
                    }
                }
            }
        }
    }

    // BFS to find the shortest path from (0, 0) to (5, 5)
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; WIDTH]; HEIGHT];

    queue.push_back((0, 0, 0)); // (x, y, path_length)
    visited[0][0] = true;

    // Directions for moving up, down, left, or right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y, dist)) = queue.pop_front() {
        // If we've reached the target, mark the path and return the length
        if x == WIDTH - 1 && y == HEIGHT - 1 {
            mark_path(&mut grid, &visited, dist);
            println!("path distance {}", &dist);
            return dist;
        }

        // Check all possible directions
        for &(dx, dy) in &directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < WIDTH && ny < HEIGHT && !visited[ny][nx] && grid[ny][nx] != '#' {
                visited[ny][nx] = true;
                queue.push_back((nx, ny, dist + 1));
            }
        }
    }

    // No valid path found
    0
}

// Mark the shortest path on the grid with '0'
fn mark_path(grid: &mut Vec<Vec<char>>, visited: &[Vec<bool>], mut dist: usize) {
    const WIDTH: usize = 6;
    const HEIGHT: usize = 6;

    let mut x = WIDTH - 1;
    let mut y = HEIGHT - 1;

    while dist > 0 {
        grid[y][x] = '0';
        dist -= 1;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for &(dx, dy) in &directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < WIDTH && ny < HEIGHT && grid[ny][nx] == '0' && visited[ny][nx] {
                x = nx;
                y = ny;
                break;
            }
        }
    }

    // Mark the starting point
    grid[0][0] = '0';
    print_grid(&grid);
}

// Debugging function to print the grid
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
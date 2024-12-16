use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

pub fn calculate_fencing_cost() -> u32 {
    // let test_map = vec![
    //     vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
    //     vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
    //     vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
    //     vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
    //     vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
    //     vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
    //     vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
    //     vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
    //     vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
    //     vec!['M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E', 'E'],
    // ];
    let input = fs::read_to_string("src/inputs/day12_q1")
        .expect("missing input file");

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };
    let mut visited = HashSet::new();
    let mut total_price = 0;

    // Explore a region using DFS
    fn explore_region(
        map: &[Vec<char>],
        visited: &mut HashSet<Position>,
        start: Position,
        rows: usize,
        cols: usize,
    ) -> (u32, u32) {
        let mut stack = vec![start];
        let mut area = 0;
        let mut perimeter = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(pos) = stack.pop() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);
            area += 1;

            for &(dx, dy) in &directions {
                let new_row = pos.row as isize + dx;
                let new_col = pos.col as isize + dy;

                if new_row < 0 || new_col < 0 || new_row >= rows as isize || new_col >= cols as isize
                {
                    // Edge of the map contributes to the perimeter
                    perimeter += 1;
                } else {
                    let neighbor = Position {
                        row: new_row as usize,
                        col: new_col as usize,
                    };

                    if map[new_row as usize][new_col as usize] == map[pos.row][pos.col] {
                        // Same region, add to stack if not visited
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    } else {
                        // Different region, contributes to the perimeter
                        perimeter += 1;
                    }
                }
            }
        }

        (area, perimeter)
    }

    // Iterate through all positions in the grid
    for row in 0..rows {
        for col in 0..cols {
            let pos = Position { row, col };
            if !visited.contains(&pos) {
                // Explore the region & calculate area & perimeter
                let (area, perimeter) = explore_region(&map, &mut visited, pos, rows, cols);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

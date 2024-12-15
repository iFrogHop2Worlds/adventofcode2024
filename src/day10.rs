use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn find_trailhead_scores<T>() -> usize {
    let input = fs::read_to_string("src/inputs/day10_q1")
        .expect("missing input file");
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10).map(|digit| digit as u8))
                .collect::<Vec<_>>()
        })
        .collect();

    if map.is_empty() || map[0].is_empty() {
        panic!("Input map is empty or malformed. Please ensure the file contains rows of integers.");
    }

    let rows = map.len();
    let cols = map[0].len();
    let mut total_trailhead_score = 0;

    fn get_neighbors(pos: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
        let (r, c) = pos;
        let mut neighbors = Vec::new();

        if r > 0 {
            neighbors.push((r - 1, c)); // Up
        }
        if r < rows - 1 {
            neighbors.push((r + 1, c)); // Down
        }
        if c > 0 {
            neighbors.push((r, c - 1)); // Left
        }
        if c < cols - 1 {
            neighbors.push((r, c + 1)); // Right
        }

        neighbors
    }

    fn bfs_flood_fill<T>(
        start: (usize, usize),
        map: &Vec<Vec<u8>>,
        rows: usize,
        cols: usize,
    ) -> usize {
        let mut score = 0;
        let mut queue = VecDeque::new();
        let mut visited_paths: HashSet<T> = HashSet::new(); // Track visited coordinates

        // Initialize BFS from the trailhead
        queue.push_back((start, vec![start])); // Position and the path taken to reach it

        while let Some((cur_pos, cur_path)) = queue.pop_front() {
            let (cur_r, cur_c) = cur_pos;
            let cur_height = map[cur_r][cur_c];

            for (nr, nc) in get_neighbors((cur_r, cur_c), rows, cols) {
                // A valid neighbor must:
                // 1) Have a height greater than the current cell
                // 2) Not have been visited in the current path
                if map[nr][nc] > cur_height && !cur_path.contains(&(nr, nc)) {
                    let mut new_path = cur_path.clone();
                    new_path.push((nr, nc));

                    if map[nr][nc] == 9 {
                        // Reached a valid end of a hiking trail
                        score += 1;
                    } else {
                        // Continue BFS flood fill with this path
                        queue.push_back(((nr, nc), new_path));
                    }
                }
            }
        }

        score
    }

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                total_trailhead_score += bfs_flood_fill::<T>((r, c), &map, rows, cols);
            }
        }
    }

    total_trailhead_score
}
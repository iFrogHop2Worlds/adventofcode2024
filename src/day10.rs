use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn find_trailhead_scores() -> usize {
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

    // Check for empty or malformed data
    if map.is_empty() || map[0].is_empty() {
        panic!("Input map is empty or malformed. Please ensure the file contains rows of integers.");
    }

    let rows = map.len();
    let cols = map[0].len();
    let mut trailhead_scores = 0;

    fn get_neighbors(pos: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (r, c) = pos;
        if r > 0 {
            neighbors.push((r - 1, c)); // Move up
        }
        if r < rows - 1 {
            neighbors.push((r + 1, c)); // Move down
        }
        if c > 0 {
            neighbors.push((r, c - 1)); // Move left
        }
        if c < cols - 1 {
            neighbors.push((r, c + 1)); // Move right
        }
        neighbors
    }

    fn bfs_trailhead_score(
        start: (usize, usize),
        map: &Vec<Vec<u8>>,
        rows: usize,
        cols: usize,
    ) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut score = 0;

        queue.push_back((start, 0)); // (position, current height)
        visited.insert(start);

        while let Some(((cur_r, cur_c), cur_height)) = queue.pop_front() {
            if map[cur_r][cur_c] == 9 {
                score += 1;
                continue;
            }

            for (nr, nc) in get_neighbors((cur_r, cur_c), rows, cols) {
                if !visited.contains(&(nr, nc)) && map[nr][nc] == cur_height + 1 {
                    queue.push_back(((nr, nc), map[nr][nc]));
                    visited.insert((nr, nc));
                }
            }
        }

        score
    }

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailhead_scores += bfs_trailhead_score((r, c), &map, rows, cols);
            }
        }
    }

    trailhead_scores
}
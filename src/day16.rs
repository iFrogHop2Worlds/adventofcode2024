use std::collections::{VecDeque, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct State {
    position: (usize, usize),
    direction: Direction,
    score: usize,
}

impl State {
    fn new(position: (usize, usize), direction: Direction, score: usize) -> Self {
        Self {
            position,
            direction,
            score,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_counterclockwise(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn move_forward(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North => x.checked_sub(1).map(|nx| (nx, y)),
            Direction::East => Some((x, y + 1)),
            Direction::South => Some((x + 1, y)),
            Direction::West => y.checked_sub(1).map(|ny| (x, ny)),
        }
    }
}
pub fn find_lowest_score() -> Option<usize> {
    let input = fs::read_to_string("src/inputs/day16_q1").expect("missing input file");
    let mut maze: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_position = maze.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&c| c == 'S').map(|j| (i, j))
    })?;
    let end_position = maze.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&c| c == 'E').map(|j| (i, j))
    })?;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start with initial state
    queue.push_back(State::new(start_position, Direction::East, 0));
    visited.insert((start_position, Direction::East));

    while let Some(state) = queue.pop_front() {
        if state.position == end_position {
            println!("Maze:\n {}", &maze.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
            return Some(state.score);
        }

        if let Some(new_position) = state.direction.move_forward(state.position) {
            if maze[new_position.0][new_position.1] != '#' && !visited.contains(&(new_position, state.direction)) {
                queue.push_back(State::new(new_position, state.direction, state.score + 1));
                visited.insert((new_position, state.direction));

                match state.direction {
                    Direction::North => maze[new_position.0][new_position.1] = '^',
                    Direction::East => maze[new_position.0][new_position.1] = '>',
                    Direction::South => maze[new_position.0][new_position.1] = 'v',
                    Direction::West => maze[new_position.0][new_position.1] = '<',
                }
            }
        }

        for &new_direction in &[
            state.direction.turn_counterclockwise(),
            state.direction.turn_clockwise(),
        ] {
            if !visited.contains(&(state.position, new_direction)) {
                queue.push_back(State::new(state.position, new_direction, state.score + 1000));
                visited.insert((state.position, new_direction));
            }
        }

    }

    None
}
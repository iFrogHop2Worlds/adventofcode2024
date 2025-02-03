use std::collections::VecDeque;
use std::fs;

pub fn race_condition_simulator() -> Option<u64> {
    let mut number_of_cheats: u64 = 0;
    let input = fs::read_to_string("src/inputs/day20_q1").expect("missing input file");
    let mut race_track: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Start and End Positions
    let start_position = race_track.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&c| c == 'S').map(|j| (i, j))
    })?;
    let end_position = race_track.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&c| c == 'E').map(|j| (i, j))
    })?;

    let base_moves = bfs(&race_track, start_position, end_position, None).unwrap_or(0);
    //println!("The base moves are: {}", base_moves);

    let reachable_walls = find_reachable_walls(&race_track, start_position);
    //println!("Reachable walls: {:?}", reachable_walls);

    for &wall in &reachable_walls {
        if let Some(rescue_moves) = bfs(&race_track, start_position, end_position, Some(wall)) {
            //println!("{:?} rescue moves: {}", wall, rescue_moves);
            if (base_moves - rescue_moves) >= 100 {
                number_of_cheats += 1;
            }
        }
    }

    println!(
        "There are: {} cheats that will save you at least 100 picoseconds",
        number_of_cheats
    );
    //print_race_track(&race_track, start_position, end_position);
    Some(number_of_cheats)
}

fn find_reachable_walls(
    race_track: &[Vec<char>],
    start: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut reachable_walls = vec![];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; race_track[0].len()]; race_track.len()];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in directions.iter() {
            let nx = x.wrapping_add(*dx as usize);
            let ny = y.wrapping_add(*dy as usize);

            if nx >= race_track.len() || ny >= race_track[0].len() || visited[nx][ny] {
                continue;
            }

            visited[nx][ny] = true;

            if race_track[nx][ny] == '#' {
                reachable_walls.push((nx, ny));
            }

            if race_track[nx][ny] == '.' || race_track[nx][ny] == 'E' {
                queue.push_back((nx, ny));
            }
        }
    }

    reachable_walls
}

fn bfs(
    race_track: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    wall_to_ignore: Option<(usize, usize)>,
) -> Option<u64> {
    let mut queue: VecDeque<((usize, usize), u64)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; race_track[0].len()]; race_track.len()];

    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps);
        }

        for (dx, dy) in directions.iter() {
            let nx = x.wrapping_add(*dx as usize);
            let ny = y.wrapping_add(*dy as usize);

            if nx >= race_track.len() || ny >= race_track[0].len() || visited[nx][ny] {
                continue;
            }

            if race_track[nx][ny] == '#' && Some((nx, ny)) != wall_to_ignore {
                continue;
            }

            visited[nx][ny] = true;
            queue.push_back(((nx, ny), steps + 1));
        }
    }

    None
}

fn print_race_track(
    race_track: &Vec<Vec<char>>,
    start_position: (usize, usize),
    end_position: (usize, usize),
) {
    println!(
        "start position {:?}, end position {:?}",
        start_position, end_position
    );
    for row in race_track {
        println!("{}", row.iter().collect::<String>());
    }
}
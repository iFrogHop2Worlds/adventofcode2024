use std::collections::HashSet;
use std::fs;

pub fn calculate_antinode_positions() -> usize {
    let input = fs::read_to_string("src/inputs/day8_q1").expect("missing input file");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: Vec<(char, usize, usize)> = Vec::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (row_index, row) in map.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if (cell.is_alphabetic() || cell.is_digit(10)) && cell != '.' {
                antennas.push((cell, row_index, col_index));
            }
        }
    }

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (freq1, row1, col1) = antennas[i];
            let (freq2, row2, col2) = antennas[j];

            if freq1 == freq2 {
                let row_step = (row2 as isize - row1 as isize).signum();
                let col_step = (col2 as isize - col1 as isize).signum();

                let distance = if row1 == row2 || col1 == col2 {
                    (row2 as isize - row1 as isize).abs().max((col2 as isize - col1 as isize).abs())
                } else {
                    (row2 as isize - row1 as isize).abs().min((col2 as isize - col1 as isize).abs())
                };

                let new_row1 = row1 as isize - distance * row_step;
                let new_col1 = col1 as isize - distance * col_step;
                let new_row2 = row2 as isize + distance * row_step;
                let new_col2 = col2 as isize + distance * col_step;
                println!("new_row1: {}", &new_row1);
                println!("new_col1: {}", &new_col1);
                println!("new_row2: {}", &new_row2);
                println!("new_col2: {}", &new_col2);

                if new_row1 >= 0 && new_row1 < map.len() as isize && new_col1 >= 0 && new_col1 < map[0].len() as isize {
                    antinodes.insert((new_row1 as usize, new_col1 as usize));
                }
                if new_row2 >= 0 && new_row2 < map.len() as isize && new_col2 >= 0 && new_col2 < map[0].len() as isize {
                    antinodes.insert((new_row2 as usize, new_col2 as usize));
                }
            }
        }
    }

    let mut visual_map = map.clone();
    for (row, col) in &antinodes {
        visual_map[*row][*col] = '#';
    }

    for row in visual_map {
        println!("{}", row.iter().collect::<String>());
    }

    antinodes.len()
}
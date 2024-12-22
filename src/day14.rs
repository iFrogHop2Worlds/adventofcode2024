pub fn predict_robots_movement() {
    let mut grid = generate_grid(103, 101);
    let robot_movements = create_movement_data(Option::from("src/inputs/day14_q1"));
    let mut current_positions: Vec<(i16, i16)> = robot_movements.iter().map(|movement| movement[0]).collect();
    //
    for iteration in 0..101 {
        println!("Iteration {}", iteration);
        if iteration == 0 {
            // Mark initial starting positions
            for &(x, y) in &current_positions {
                let (wrapped_x, wrapped_y) = wrap_around(x, y, &grid);
                grid[wrapped_y][wrapped_x] += 1;
            }
        } else {
            // Move robots based on their velocity
            for (i, position) in current_positions.iter_mut().enumerate() {
                let velocity = robot_movements[i][1];
                let new_position = (
                    position.0 + velocity.0,
                    position.1 + velocity.1,
                );
                // Wrap-around
                let (current_x, current_y) = wrap_around(position.0, position.1, &grid);
                let (new_x, new_y) = wrap_around(new_position.0, new_position.1, &grid);
                // update values/move
                grid[current_y][current_x] -= 1;
                *position = new_position;
                grid[new_y][new_x] += 1;
            }
        }
        // Print grid for debugging
        print_grid(&grid);
    }

    let result = compute_quadrant_product(&grid);
    println!("Product of quadrant sums: {}", result);
}

// Function to generate a grid
fn generate_grid(height: usize, width: usize) -> Vec<Vec<i8>> {
    vec![vec![0; width]; height]
}

fn create_movement_data(input_file: Option<&str>) -> Vec<Vec<(i16, i16)>> {
    if let Some(file) = input_file.filter(|&f| !f.is_empty()) {
        let content = std::fs::read_to_string(file).expect("Failed to read file");
        content
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    let p_part = parts[0].trim_start_matches("p=").split(',').collect::<Vec<&str>>();
                    let v_part = parts[1].trim_start_matches("v=").split(',').collect::<Vec<&str>>();
                    if p_part.len() == 2 && v_part.len() == 2 {
                        if let (Ok(px), Ok(py), Ok(vx), Ok(vy)) = (
                            p_part[0].parse::<i16>(),
                            p_part[1].parse::<i16>(),
                            v_part[0].parse::<i16>(),
                            v_part[1].parse::<i16>(),
                        ) {
                            return vec![(px, py), (vx, vy)];
                        }
                    }
                }
                vec![]
            })
            .filter(|v| !v.is_empty())
            .collect()
    } else {
        vec![
            vec![(0, 4), (3, -3)],
            vec![(6, 3), (-1, -3)],
            vec![(10, 3), (-1, 2)],
            vec![(2, 0), (2, -1)],
            vec![(0, 0), (1, 3)],
            vec![(3, 0), (-2, -2)],
            vec![(7, 6), (-1, -3)],
            vec![(3, 0), (-1, -2)],
            vec![(9, 3), (2, 3)],
            vec![(7, 3), (-1, 2)],
            vec![(2, 4), (2, -3)],
            vec![(9, 5), (-3, -3)],
        ]
    }
}

fn wrap_around(x: i16, y: i16, grid: &[Vec<i8>]) -> (usize, usize) {
    let height = grid.len() as i16;
    let width = grid[0].len() as i16;
    // modular arithmetic to wrap around the position
    let wrapped_x = ((x % width) + width) % width;
    let wrapped_y = ((y % height) + height) % height;
    (wrapped_x as usize, wrapped_y as usize)
}

//debugging
fn print_grid(grid: &[Vec<i8>]) {
    for row in grid {
        println!("{:?}", row);
    }
    println!();
}

fn compute_quadrant_product(grid: &[Vec<i8>]) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    let mid_y = height / 2;
    let mid_x = width / 2;

    let sum_q1: i32 = grid[..mid_y]
        .iter()
        .map(|row| row[..mid_x-1].iter().map(|&val| val as i32).sum::<i32>())
        .sum();

    let sum_q2: i32 = grid[..mid_y]
        .iter()
        .map(|row| row[mid_x..].iter().map(|&val| val as i32).sum::<i32>())
        .sum();

    let sum_q3: i32 = grid[mid_y+1..]
        .iter()
        .map(|row| row[..mid_x].iter().map(|&val| val as i32).sum::<i32>())
        .sum();

    let sum_q4: i32 = grid[mid_y+1..]
        .iter()
        .map(|row| row[mid_x+1..].iter().map(|&val| val as i32).sum::<i32>())
        .sum();


    println!("Sum of quadrants: {:?} {:?} {:?} {:?} ", sum_q1, sum_q2, sum_q3, sum_q4);


    sum_q1 * sum_q2 * sum_q3 * sum_q4
}
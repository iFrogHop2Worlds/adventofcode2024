use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: Position,
    rows: usize,
    cols: usize,
}

impl Warehouse {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut robot = Position { x: 0, y: 0 };

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == '@' {
                    robot = Position { x, y };
                    break;
                }
            }
        }

        Self {
            grid,
            robot,
            rows,
            cols,
        }
    }

    pub fn step(&mut self, direction: char) {
        let (dx, dy) = match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0), // Ignore invalid directions
        };

        let new_robot_pos = Position {
            x: (self.robot.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
            y: (self.robot.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
        };

        // Validate if it's a box
        let is_box = |x: usize, y: usize| {
            self.grid[y][x] == '[' && x + 1 < self.cols && self.grid[y][x + 1] == ']'
                || self.grid[y][x] == ']' && x >= 1 && self.grid[y][x - 1] == '['
        };

        // Collect the chain of sliding boxes and validate their movement
        if is_box(new_robot_pos.x, new_robot_pos.y) {
            let mut boxes_to_move = vec![];
            let mut current_pos = new_robot_pos;

            // Traverse all connected boxes and gather their positions
            loop {
                let x = current_pos.x;
                let y = current_pos.y;

                if is_box(x, y) {
                    if self.grid[y][x] == '[' {
                        boxes_to_move.push((Position { x, y }, Position { x: x + 1, y }));
                    } else if self.grid[y][x] == ']' && x >= 1 && self.grid[y][x - 1] == '[' {
                        boxes_to_move.push((Position { x: x - 1, y }, Position { x, y }));
                    }

                    current_pos = Position {
                        x: (current_pos.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                        y: (current_pos.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                    };
                } else {
                    break;
                }
            }

            // Validate if all the boxes can move in the given direction
            let mut valid_move = true;
            for (left, right) in &boxes_to_move {
                let next_left = Position {
                    x: (left.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                    y: (left.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                };
                let next_right = Position {
                    x: (right.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                    y: (right.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                };

                // Check if the spaces are free
                if !(self.grid[next_left.y][next_left.x] == '.' && self.grid[next_right.y][next_right.x] == '.') {
                    valid_move = false;
                    break;
                }
            }

            // Move boxes if valid
            if valid_move {
                for (left, right) in boxes_to_move.iter().rev() {
                    let next_left = Position {
                        x: (left.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                        y: (left.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                    };
                    let next_right = Position {
                        x: (right.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                        y: (right.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                    };

                    self.grid[left.y][left.x] = '.';
                    self.grid[right.y][right.x] = '.';
                    self.grid[next_left.y][next_left.x] = '[';
                    self.grid[next_right.y][next_right.x] = ']';
                }

                // Update robot position
                self.grid[new_robot_pos.y][new_robot_pos.x] = '@';
                self.grid[self.robot.y][self.robot.x] = '.';
                self.robot = new_robot_pos;
            }
        }
        // If moving into an empty space
        else if self.grid[new_robot_pos.y][new_robot_pos.x] == '.' {
            self.grid[new_robot_pos.y][new_robot_pos.x] = '@';
            self.grid[self.robot.y][self.robot.x] = '.';
            self.robot = new_robot_pos;
        }
    }

    pub fn sum_gps_coordinates(&self) -> usize {
        let mut sum = 0;

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == '[' {
                    sum += 100 * (y) + (x);
                }
            }
        }

        sum
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
}

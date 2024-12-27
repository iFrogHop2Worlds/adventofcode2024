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

        // handle multiple boxes in succession
        if self.grid[new_robot_pos.y][new_robot_pos.x] == 'O' {
            // try to move multiple boxes
            let mut boxes_to_move = vec![];
            let mut current_pos = new_robot_pos;

            // collect consecutive boxes in the moving direction
            loop {
                if self.grid[current_pos.y][current_pos.x] == 'O' {
                    boxes_to_move.push(current_pos);
                    current_pos = Position {
                        x: (current_pos.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                        y: (current_pos.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                    };
                } else {
                    break;
                }
            }

            // if there's a free space for all boxes
            if self.grid[current_pos.y][current_pos.x] == '.' {
                // move boxes
                while let Some(box_pos) = boxes_to_move.pop() {
                    let next_pos = Position {
                        x: (box_pos.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                        y: (box_pos.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
                    };
                    self.grid[next_pos.y][next_pos.x] = 'O';
                    self.grid[box_pos.y][box_pos.x] = '.';
                }

                // move the robot
                self.grid[new_robot_pos.y][new_robot_pos.x] = '@';
                self.grid[self.robot.y][self.robot.x] = '.';
                self.robot = new_robot_pos;
            }
        } else if self.grid[new_robot_pos.y][new_robot_pos.x] == '.' {
            // move the robot
            self.grid[new_robot_pos.y][new_robot_pos.x] = '@';
            self.grid[self.robot.y][self.robot.x] = '.';
            self.robot = new_robot_pos;
        }
        if self.grid[new_robot_pos.y][new_robot_pos.x] == '#' {
            // wall, cannot move
            return;
        }

        if self.grid[new_robot_pos.y][new_robot_pos.x] == 'O' {
            // try to push the box
            let new_box_pos = Position {
                x: (new_robot_pos.x as isize + dx).clamp(0, (self.cols - 1) as isize) as usize,
                y: (new_robot_pos.y as isize + dy).clamp(0, (self.rows - 1) as isize) as usize,
            };

            if self.grid[new_box_pos.y][new_box_pos.x] == '.' {
                // move the box
                self.grid[new_box_pos.y][new_box_pos.x] = 'O';
                self.grid[new_robot_pos.y][new_robot_pos.x] = '@';
                self.grid[self.robot.y][self.robot.x] = '.';
                self.robot = new_robot_pos;
            }
        } else if self.grid[new_robot_pos.y][new_robot_pos.x] == '.' {
            // move the robot
            self.grid[new_robot_pos.y][new_robot_pos.x] = '@';
            self.grid[self.robot.y][self.robot.x] = '.';
            self.robot = new_robot_pos;
        }
    }

    pub fn sum_gps_coordinates(&self) -> usize {
        let mut sum = 0;

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'O' {
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

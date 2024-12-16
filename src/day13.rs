use std::fs;

#[derive(Debug, Clone)]
struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    target_x: i64,
    target_y: i64,
}

pub fn solve_claw_machines() -> (usize, i64) {
    let input = fs::read_to_string("src/inputs/day13_q1")
        .expect("missing input file");

    let machines = parse_claw_machines(&input);

    // Print the parsed machines for debugging
    for machine in &machines {
        println!("{:?}", machine);
    }
    // let machines = vec![
    //     ClawMachine {
    //         ax: 94,
    //         ay: 34,
    //         bx: 22,
    //         by: 67,
    //         target_x: 8400,
    //         target_y: 5400,
    //     },
    //     ClawMachine {
    //         ax: 26,
    //         ay: 66,
    //         bx: 67,
    //         by: 21,
    //         target_x: 12748,
    //         target_y: 12176,
    //     },
    //     ClawMachine {
    //         ax: 17,
    //         ay: 86,
    //         bx: 84,
    //         by: 37,
    //         target_x: 7870,
    //         target_y: 6450,
    //     },
    //     ClawMachine {
    //         ax: 69,
    //         ay: 23,
    //         bx: 27,
    //         by: 71,
    //         target_x: 18641,
    //         target_y: 10279,
    //     },
    // ];
    let mut total_tokens_spent = 0;
    let mut prizes_won = 0;

    for machine in machines {
        let mut min_tokens: Option<i64> = None;

        // nested loop to try all combinations of A and B presses
        for a_presses in 0..=100 {
            for b_presses in 0..=100 {
                let nx = a_presses * machine.ax + b_presses * machine.bx;
                let ny = a_presses * machine.ay + b_presses * machine.by;

                if nx == machine.target_x && ny == machine.target_y {
                    let cost = a_presses * 3 + b_presses;
                    match min_tokens {
                        Some(current_min) => min_tokens = Some(current_min.min(cost)),
                        None => min_tokens = Some(cost),
                    }
                }
            }
        }

        if let Some(tokens) = min_tokens {
            prizes_won += 1;
            total_tokens_spent += tokens;
        }
    }

    (prizes_won, total_tokens_spent)
}

fn parse_claw_machines(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();

    // Split the input into groups of three lines
    let lines: Vec<&str> = input.lines().filter(|line| !line.trim().is_empty()).collect();
    for chunk in lines.chunks(3) {
        if let [a_line, b_line, prize_line] = chunk {

            let (ax, ay) = parse_button_line(a_line);
            let (bx, by) = parse_button_line(b_line);
            let (target_x, target_y) = parse_prize_line(prize_line);

            //ClawMachine struct 2 add it to the vector
            machines.push(ClawMachine {
                ax: ax,
                ay: ay,
                bx: bx,
                by: by,
                target_x: target_x,
                target_y: target_y,
            });
        }
    }

    machines
}

use regex::Regex;

fn parse_button_line(line: &str) -> (i64, i64) {
    println!("Parsing line: {}", &line);

    let re = Regex::new(r"X\+(-?\d+), Y\+(-?\d+)").expect("Failed to compile regex");
    if let Some(caps) = re.captures(line) {
        let x: i64 = caps[1].parse().expect("Invalid x value in button line");
        let y: i64 = caps[2].parse().expect("Invalid y value in button line");
        (x, y)
    } else {
        panic!("Invalid line format: {}", line);
    }
}

fn parse_prize_line(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(['X', 'Y', '=', ',', ':'].as_ref())
        .filter(|p| !p.is_empty())
        .collect();
    if parts.len() < 3 {
        panic!("Unexpected prize line format: {}", line);
    }
    let x: i64 = parts[1].trim().parse().expect("Invalid target_x in prize line");
    let y: i64 = parts[2].trim().parse().expect("Invalid target_y in prize line");
    (x, y)
}
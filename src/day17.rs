pub fn three_bit_computer() {
    let program = vec![2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0];

    fn simulate(register_a: i64, program: &[i64]) -> Vec<i64> {
        let mut register_a: i64 = register_a;
        let mut register_b: i64 = 0;
        let mut register_c: i64 = 0;
        let mut instruction_pointer = 0;
        let mut output = Vec::new();

        while instruction_pointer < program.len() {
            let opcode = program[instruction_pointer];
            let operand = program[instruction_pointer + 1];

            match opcode {
                0 => {
                    // adv
                    let denominator = match operand {
                        0 => 1,
                        1 => 2,
                        2 => 4,
                        3 => 8,
                        4 => 2_i64.pow(register_a as u32),
                        5 => 2_i64.pow(register_b as u32),
                        6 => 2_i64.pow(register_c as u32),
                        _ => panic!("Invalid combo operand for adv"),
                    };
                    register_a /= denominator;
                }
                1 => {
                    // bxl
                    register_b ^= operand;
                }
                2 => {
                    // bst
                    let combo_value = match operand {
                        0..=3 => operand,
                        4 => register_a,
                        5 => register_b,
                        6 => register_c,
                        _ => panic!("Invalid combo operand for bst"),
                    };
                    register_b = combo_value % 8;
                }
                3 => {
                    // jnz
                    if register_a != 0 {
                        instruction_pointer = operand as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc
                    register_b ^= register_c;
                }
                5 => {
                    // out
                    let value = match operand {
                        0..=3 => operand,
                        4 => register_a,
                        5 => register_b,
                        6 => register_c,
                        _ => panic!("Invalid combo operand for out"),
                    };
                    output.push(value % 8);
                }
                6 => {
                    // bdv
                    let denominator = match operand {
                        0 => 1,
                        1 => 2,
                        2 => 4,
                        3 => 8,
                        4 => 2_i64.pow(register_a as u32),
                        5 => 2_i64.pow(register_b as u32),
                        6 => 2_i64.pow(register_c as u32),
                        _ => panic!("Invalid combo operand for bdv"),
                    };
                    register_b = register_a / denominator;
                }
                7 => {
                    // cdv
                    let denominator = match operand {
                        0 => 1,
                        1 => 2,
                        2 => 4,
                        3 => 8,
                            4 => 2_i64.pow(register_a as u32),
                        5 => 2_i64.pow(register_b as u32),
                        6 => 2_i64.pow(register_c as u32),
                        _ => panic!("Invalid combo operand for cdv"),
                    };
                    register_c = register_a / denominator;
                }
                _ => panic!("Invalid opcode encountered"),
            }

            instruction_pointer += 2;
        }

        output
    }

    let mut register_a = 47719761;
    loop {
        let output = simulate(register_a, &program);

        if output == program {
            println!("The lowest initial value for register A is: {}", register_a);
            break;
        }

        register_a += 1;
    }
}
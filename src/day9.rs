use std::fs;

pub fn free_disk_space() -> usize {
    let disk_map = fs::read_to_string("src/inputs/day9_q1").expect("missing input file");
    let mut blocks: Vec<char> = Vec::new();
    let chars: Vec<char> = disk_map.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let file_len = match chars[i].to_digit(10) {
            Some(digit) => digit as usize,
            None => continue, // Skip if parsing fails
        };
        let space_len = if i + 1 < chars.len() {
            match chars[i + 1].to_digit(10) {
                Some(digit) => digit as usize,
                None => continue, // Skip if parsing fails
            }
        } else {
            0
        };
        blocks.extend(std::iter::repeat((i / 2).to_string().chars().next().unwrap()).take(file_len));
        blocks.extend(std::iter::repeat('.').take(space_len));
        i += 2;
    }

    if blocks.is_empty() {
        return 0;
    }

    blocks = swap_blocks(blocks);
    println!("Bloxes: {}", blocks.len());
    println!("Blocks: {:#?}", blocks);

    let mut checksum = 0;
    for i in 0..blocks.len() {
        if blocks[i] != '.' {
            checksum += i * (blocks[i].to_digit(10).unwrap() as usize);
        }
    }

    checksum
}

fn swap_blocks(mut blocks: Vec<char>) -> Vec<char> {
    let mut read_idx = blocks.len() - 1; // Start from the last index
    for write_idx in (0..blocks.len()) {
        if blocks[write_idx].is_ascii_digit() {
            continue;
        }
        if blocks[read_idx] == '.' {
            while read_idx > 0 && blocks[read_idx] == '.' {
                read_idx -= 1;
            }
        }

        if read_idx > write_idx && blocks[write_idx] == '.' && !blocks[write_idx].is_ascii_digit() {
            blocks.swap(write_idx, read_idx);
        }
        if read_idx > 0 {
            read_idx -= 1;
        }
    }
    blocks
}
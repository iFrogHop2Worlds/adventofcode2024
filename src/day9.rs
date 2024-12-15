use std::fs;
//I think our error is not handling 2char digits?
pub fn free_disk_space() -> u64 {
    let disk_map = fs::read_to_string("src/inputs/day9_q1").expect("missing input file");
    let mut blocks: Vec<char> = Vec::new();
    let chars: Vec<char> = disk_map.chars().collect();

    let mut i: u64 = 0;
    while (i as usize) < chars.len() {
        // Parse the current character as a number
        let file_len = match chars[i as usize].to_digit(10) {
            Some(digit) => digit as u64,
            None => {
                i += 1; // Move to the next character if parsing fails
                continue;
            }
        };

        // Parse the next character if it exists
        let space_len = if ((i + 1) as usize) < chars.len() {
            match chars[(i + 1) as usize].to_digit(10) {
                Some(digit) => digit as u64,
                None => {
                    i += 2; // Skip both characters if the second parsing fails
                    continue;
                }
            }
        } else {
            0 // Default to 0 if no next character
        };

        // Build the blocks vector based on file and space lengths
        blocks.extend(std::iter::repeat((i / 2).to_string().chars().next().unwrap()).take(file_len as usize));
        blocks.extend(std::iter::repeat('.').take(space_len as usize));

        i += 2;
        println!("i: {}", i);// testing
    }

    if blocks.is_empty() {
        return 0;
    }
    println!("Bloxes: {}", blocks.len());
    println!("Blocks: {:#?}", blocks);
    blocks = swap_blocks(blocks);
    println!("~~~~~~~~~~~~~~~~~~");
    //println!("Blocks: {:#?}", blocks);

    let mut checksum: u64 = 0;
    for (i, block) in blocks.iter().enumerate() {
        if *block != '.' {
            checksum += i as u64 * (block.to_digit(10).unwrap() as u64);
        }
    }

    checksum
}

// Shuffles values from right to left.
// Only placing values in empty spaces
fn swap_blocks(mut blocks: Vec<char>) -> Vec<char> {
    let mut read_idx: u64 = (blocks.len() - 1) as u64; // Start from the last index
    for write_idx in 0..blocks.len() {
        if blocks[write_idx].is_ascii_digit() {
            continue;
        }
        if blocks[read_idx as usize] == '.' {
            while read_idx > write_idx as u64 && blocks[read_idx as usize] == '.' {
                read_idx -= 1;
            }
        }

        if read_idx > write_idx as u64 && blocks[write_idx] == '.' && !blocks[write_idx].is_ascii_digit() {
            blocks.swap(write_idx, read_idx as usize);
        }
        if read_idx > 0 {
            read_idx -= 1;
        }
    }
    blocks
}
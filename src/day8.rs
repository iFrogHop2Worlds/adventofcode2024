// use std::collections::HashSet;
// use std::fs;
// use nom::{
//     character::complete::{anychar, not_line_ending, none_of},
//     multi::many1,
//     sequence::preceded,
//     IResult, bytes::complete::tag,
// };
//
// pub type Span<'a> = &'a str;
//
// const INPUT_FILE_PATH: &str = "src/inputs/day8_q1";
// const FAILED_TO_READ_INPUT: &str = "Failed to read input";
//
// fn read_input(file_path: &str) -> Result<String, String> {
//     fs::read_to_string(file_path).map_err(|e| format!("Error reading input file: {}", e))
// }
//
// fn get_bounds(input: &str) -> (std::ops::Range<usize>, std::ops::Range<usize>) {
//     let height = input.lines().count();
//     let width = input.lines().next().unwrap().len();
//     (0..height, 0..width)
// }
//
// fn calculate_unique_antennas(antennas: &[(char, usize, usize)], x_bound: std::ops::Range<usize>, y_bound: std::ops::Range<usize>) -> HashSet<(usize, usize)> {
//     antennas.iter()
//         .position(|&(c, _, _)| c == 'a')
//         .map(|pos| {
//             let pair = &antennas[pos..pos+2];
//             let diff_row = pair[0].1 as isize - pair[1].1 as isize;
//             let diff_col = pair[0].2 as isize - pair[1].2 as isize;
//             vec![
//                 ((pair[0].1 as isize + diff_row) as usize, (pair[0].2 as isize + diff_col) as usize),
//                 ((pair[1].1 as isize - diff_row) as usize, (pair[1].2 as isize - diff_col) as usize),
//             ]
//         })
//         .into_iter()
//         .flatten()
//         .filter(|&(x, y)| x_bound.contains(&x) && y_bound.contains(&y))
//         .collect()
// }
//
// pub fn calculate_antinode_positions() -> usize {
//     let input = read_input(INPUT_FILE_PATH).expect(FAILED_TO_READ_INPUT);
//     let (y_bound, x_bound) = get_bounds(&input);
//     let (_, antennas) = parse(&input).expect("Failed to parse input");
//     let unique_antennas = calculate_unique_antennas(&antennas, x_bound, y_bound);
//     unique_antennas.len()
// }
//
// fn alphanum_pos(input: Span) -> IResult<Span, (char, usize, usize)> {
//     let (input, _) = not_line_ending(input)?;
//     let y = input.lines().count() - 1;
//     let x = input.len() - input.trim_start_matches(char::is_whitespace).len();
//     let (input, c) = anychar(input)?;
//     Ok((input, (c, y, x)))
// }
//
// fn parse(input: Span) -> IResult<Span, Vec<(char, usize, usize)>> {
//     many1(
//         preceded(
//             tag("||"),
//             alphanum_pos
//         )
//     )(input)
// }
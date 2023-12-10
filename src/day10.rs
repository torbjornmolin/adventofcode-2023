use regex::Regex;
use std::{fs, thread::current};

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

const VERTICAL_PIPE: char = '|';
const HORIZONTAL_PIPE: char = '-';
const BEND_90_NORTH_EAST: char = 'L';
const BEND_90_NORTH_WEST: char = 'J';
const BEND_90_SOUTH_WEST: char = '7';
const BEND_90_SOUTH_EAST: char = 'F';

#[derive(Clone, Copy)]
pub struct Day10;

impl Day10 {
    const PATH: &'static str = "input/day10.txt";
    pub fn run_part_1(&self) {
        let contents =
            fs::read_to_string(Self::PATH).expect("Should have been able to read the file");

        println!("Part 1 result: {}", self.sum_part_1(&contents));
    }
    pub fn run_part_2(&self) {
        let contents =
            fs::read_to_string(Self::PATH).expect("Should have been able to read the file");

        println!("Part 2 result: {}", self.sum_part_2(&contents));
    }

    pub fn sum_part_1(self, content: &str) -> i64 {
        let line_length = content
            .lines()
            .nth(0)
            .expect("Could not read first line")
            .len();

        let start_pos = content.find("S").expect("Unable to find start position");

        //print_map(content, start_pos, 0);
        let mut steps = 1;
        let mut current_position = get_first_step_position(start_pos, content, line_length);
        let mut last_position = start_pos;
        loop {
            //print_map(content, current_position, steps);
            let next = get_next_position(current_position, last_position, content, line_length);
            last_position = current_position;
            current_position = next;

            steps += 1;
            if content
                .chars()
                .nth(current_position)
                .expect("Unable to get current char")
                == 'S'
            {
                break;
            }
        }

        if steps % 2 == 1 {
            return steps / 2 + 1;
        }
        steps / 2
    }

    pub fn sum_part_2(self, content: &str) -> i64 {
        0
    }
}

fn print_map(content: &str, current_position: usize, step_no: i64) {
    let mut new = String::from(content);
    new.replace_range(current_position..current_position + 1, "#");
    let line_length = new.lines().collect::<Vec<&str>>().len();

    let current_line_index = current_position / line_length;

    let lines = new.lines();
    println!("Step: {}", step_no);
    //for l in lines.skip(current_line_index - 2).take(3) {
    for l in lines {
        println! {"{}", l};
    }
    println!("-------")
}

fn get_char_and_pos(start_position: usize, offset: i32, content: &str) -> Option<(usize, char)> {
    if start_position as i32 + offset < 0 {
        return None;
    }
    let position = (start_position as i32 + offset) as usize;
    if content.len() < position {
        return None;
    }
    let char_at_pos = content.chars().nth(position).expect("unable to get char");
    Some((position, char_at_pos))
}
/// Get the position of the first non-start step
fn get_first_step_position(start_pos: usize, content: &str, line_length: usize) -> usize {
    let left: i32 = start_pos as i32 - 1;
    let right: i32 = start_pos as i32 + 1;
    let up: i32 = start_pos as i32 - line_length as i32 - 1;
    let down: i32 = start_pos as i32 + line_length as i32 + 1;
    // Walk North
    match get_char_and_pos(start_pos, line_length as i32 * -1 - 1, content) {
        Some((pos, VERTICAL_PIPE)) => return pos,
        Some((pos, BEND_90_SOUTH_EAST)) => return pos,
        Some((pos, BEND_90_SOUTH_WEST)) => return pos,
        Some((_, _)) => (),
        None => (),
    }
    // Walk South
    match get_char_and_pos(start_pos, line_length as i32 + 1, content) {
        Some((pos, VERTICAL_PIPE)) => return pos,
        Some((pos, BEND_90_NORTH_EAST)) => return pos,
        Some((pos, BEND_90_NORTH_WEST)) => return pos,
        Some((_, _)) => (),
        None => (),
    }
    // Walk East
    match get_char_and_pos(start_pos, 1, content) {
        Some((pos, VERTICAL_PIPE)) => return pos,
        Some((pos, BEND_90_NORTH_WEST)) => return pos,
        Some((pos, BEND_90_SOUTH_WEST)) => return pos,
        Some((_, _)) => (),
        None => (),
    }
    // Walk West
    match get_char_and_pos(start_pos, -1, content) {
        Some((pos, VERTICAL_PIPE)) => return pos,
        Some((pos, BEND_90_NORTH_EAST)) => return pos,
        Some((pos, BEND_90_SOUTH_EAST)) => return pos,
        Some((_, _)) => (),
        None => (),
    }
    panic!("No match");
}

fn get_next_position(
    current_position: usize,
    last_position: usize,
    content: &str,
    line_length: usize,
) -> usize {
    let (_, current_char) = get_char_and_pos(current_position, 0, content)
        .expect("Unable to get char at current index");

    let left: i32 = current_position as i32 - 1;
    let right: i32 = current_position as i32 + 1;
    let up: i32 = current_position as i32 - line_length as i32 - 1;
    let down: i32 = current_position as i32 + line_length as i32 + 1;

    match current_char {
        VERTICAL_PIPE => match up == last_position as i32 {
            true => return down as usize,
            false => return up as usize,
        },
        HORIZONTAL_PIPE => match left == last_position as i32 {
            true => return right as usize,
            false => return left as usize,
        },
        BEND_90_NORTH_EAST => match right == last_position as i32 {
            true => return up as usize,
            false => return right as usize,
        },
        BEND_90_NORTH_WEST => match left == last_position as i32 {
            true => return up as usize,
            false => return left as usize,
        },
        BEND_90_SOUTH_WEST => match left == last_position as i32 {
            true => return down as usize,
            false => return left as usize,
        },
        BEND_90_SOUTH_EAST => match right == last_position as i32 {
            true => return down as usize,
            false => return right as usize,
        },

        _ => (),
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_part1_example_data_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let sut = Day10 {};
        assert_eq!(4, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part1_example_data_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let sut = Day10 {};
        assert_eq!(8, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let sut = Day10 {};
        assert_eq!(2, sut.sum_part_2(input));
    }
}

use std::fs;

use colored::Colorize;
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
        let mut final_map = String::from(content);
        loop {
            //print_map(content, current_position, steps);
            let next = get_next_position(current_position, last_position, content, line_length);
            last_position = current_position;
            current_position = next;

            final_map.replace_range(current_position..current_position + 1, "#");
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
        println!("{}", final_map);
        if steps % 2 == 1 {
            return steps / 2 + 1;
        }
        steps / 2
    }

    pub fn get_looped_map(self, content: &str) -> String {
        let line_length = content
            .lines()
            .nth(0)
            .expect("Could not read first line")
            .len();

        let start_pos = content.find("S").expect("Unable to find start position");

        let mut current_position = get_first_step_position(start_pos, content, line_length);
        let mut last_position = start_pos;
        let mut final_map = String::from(content);
        loop {
            final_map.replace_range(current_position..current_position + 1, "#");

            let next = get_next_position(current_position, last_position, content, line_length);
            last_position = current_position;
            current_position = next;

            if content
                .chars()
                .nth(current_position)
                .expect("Unable to get current char")
                == 'S'
            {
                final_map.replace_range(current_position..current_position + 1, "#");
                break;
            }
        }
        final_map
    }

    pub fn sum_part_2(self, content: &str) -> i64 {
        let mut result = 0;
        let map_with_path = self.get_looped_map(content);
        for (line_index, line) in content.lines().enumerate() {
            let mut line_copy = String::from(line);
            let mut inside = false;
            let mut seen_f = false;
            let mut seen_l = false;
            for (index, c) in line.chars().enumerate() {
                let is_map_char: bool = map_with_path
                    .lines()
                    .nth(line_index)
                    .expect("Unable to get line")
                    .chars()
                    .nth(index)
                    .expect("failed to get line map char")
                    == '#';
                if !is_map_char {
                    seen_f = false;
                    seen_l = false;
                }
                if is_map_char {
                    match c {
                        // Switch at passage. | is passage and also lines like so:
                        // L - 7
                        // F - J
                        '|' => {
                            inside = !inside;
                            continue;
                        }
                        'F' => {
                            seen_f = true;
                            seen_l = false;
                            continue;
                        }
                        'L' | 'S' => {
                            // | S is ugly hack for my input. Could write code to detect what S should be.
                            seen_l = true;
                            seen_f = false;
                            continue;
                        }
                        'J' => {
                            if seen_f {
                                seen_f = false;
                                inside = !inside;
                                continue;
                            }
                        }
                        '7' => {
                            if seen_l {
                                seen_l = false;
                                inside = !inside;
                                continue;
                            }
                        }
                        _ => (),
                    }
                }
                if !is_map_char && inside {
                    result += 1;
                    line_copy.replace_range(index..index + 1, "I");
                }
            }
            for (index, c) in line_copy.char_indices() {
                let is_map_char: bool = map_with_path
                    .lines()
                    .nth(line_index)
                    .expect("Unable to get line")
                    .chars()
                    .nth(index)
                    .expect("failed to get line map char")
                    == '#';
                if is_map_char {
                    print!("{}", String::from(c).red());
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
        result
    }
}
fn print_map(content: &str, current_position: usize, step_no: i64) {
    let mut new = String::from(content);
    new.replace_range(current_position..current_position + 1, "#");
    let line_length = new.lines().collect::<Vec<&str>>().len();

    let lines = new.lines();
    println!("Step: {}", step_no);
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
    fn correct_part2_example_data_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let sut = Day10 {};
        assert_eq!(4, sut.sum_part_2(input));
    }
    #[test]
    fn correct_part2_example_data_2() {
        let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
        let sut = Day10 {};
        assert_eq!(4, sut.sum_part_2(input));
    }
    #[test]
    fn correct_part2_example_data_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let sut = Day10 {};
        assert_eq!(10, sut.sum_part_2(input));
    }
    #[test]
    fn correct_part2_example_data_4() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let sut = Day10 {};
        assert_eq!(8, sut.sum_part_2(input));
    }
}

use std::fs;

use crate::utils::Utils;

#[derive(Clone, Copy)]
pub struct Day3;

impl Day3 {
    const PATH: &str = "input/day3.txt";
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

    pub fn sum_part_1(self, content: &str) -> u32 {
        let mut result: u32 = 0;

        let map = Utils::text_to_2d_array(content);

        for (line_no, map_line) in map.iter().enumerate() {
            let mut has_adj_symbol = false;
            let mut current_number = Vec::<char>::new();
            for (char_index, c) in map_line.iter().enumerate() {
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        current_number.push(c.clone());
                        if !has_adj_symbol {
                            has_adj_symbol = check_adjacent_symbol(&map, line_no, char_index);
                        }
                    }

                    _ => {
                        if current_number.len() > 0 && has_adj_symbol {
                            let number: String = current_number.into_iter().collect();
                            let number: u32 =
                                number.parse().expect("Unable to parse string to number");

                            result += number;
                        }
                        current_number = Vec::<char>::new();
                        has_adj_symbol = false;
                    }
                }
            }
            if current_number.len() > 0 && has_adj_symbol {
                let number: String = current_number.into_iter().collect();
                let number: u32 = number.parse().expect("Unable to parse string to number");
                result += number;
            }
        }

        result
    }

    pub fn sum_part_2(self, content: &str) -> u32 {
        let mut result: u32 = 0;

        result
    }
}

fn check_adjacent_symbol(map: &Vec<Vec<char>>, line_no: usize, char_index: usize) -> bool {
    // line above
    if line_no > 0 {
        let target_line = &map[line_no - 1];
        if char_index > 0 && is_symbol(target_line[char_index - 1]) {
            return true;
        }
        if is_symbol(target_line[char_index]) {
            return true;
        }
        if char_index + 1 < target_line.len() && is_symbol(target_line[char_index + 1]) {
            return true;
        }
    }
    // adjacent
    if char_index > 0 && is_symbol(map[line_no][char_index - 1]) {
        return true;
    }
    if char_index + 1 < map[line_no].len() && is_symbol(map[line_no][char_index + 1]) {
        return true;
    }

    //  line bellow
    if line_no + 1 < map.len() {
        let target_line = &map[line_no + 1];
        if char_index > 0 && is_symbol(target_line[char_index - 1]) {
            return true;
        }
        if is_symbol(target_line[char_index]) {
            return true;
        }
        if char_index + 1 < target_line.len() && is_symbol(target_line[char_index + 1]) {
            return true;
        }
    }
    false
}

fn is_symbol(char: char) -> bool {
    if char.is_digit(10) {
        return false;
    }
    if char == '.' {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "467..114..
...*......
..35...633
.......#..
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let sut = Day3 {};
        assert_eq!(4361, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let sut = Day3 {};
        assert_eq!(467835, sut.sum_part_2(input));
    }
}

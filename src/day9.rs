use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Clone, Copy)]
pub struct Day9;

impl Day9 {
    const PATH: &'static str = "input/day9.txt";
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

    pub fn sum_part_1(self, content: &str) -> u64 {
        0
    }

    pub fn sum_part_2(self, content: &str) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "";

        let sut = Day9 {};
        assert_eq!(6440, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "";
        let sut = Day9 {};
        assert_eq!(5905, sut.sum_part_2(input));
    }
}

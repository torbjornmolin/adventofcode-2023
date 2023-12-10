use regex::Regex;
use std::fs;

#[derive(Clone, Copy)]
pub struct Day11;

impl Day11 {
    const PATH: &'static str = "input/day11.txt";
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
        0
    }

    pub fn sum_part_2(self, content: &str) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "0 3 6 11 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let sut = Day11 {};
        assert_eq!(114, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "0 3 6 11 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let sut = Day11 {};
        assert_eq!(2, sut.sum_part_2(input));
    }
}

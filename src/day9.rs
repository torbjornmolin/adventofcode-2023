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

    pub fn sum_part_1(self, content: &str) -> i64 {
        let mut results = Vec::new();
        for line in content.lines() {
            //println!("next");
            let numbers = numbers_from_line(line);
            let mut last_digits: Vec<i64> = Vec::new();
            last_digits.push(numbers[numbers.len() - 1]);
            let mut current_iter = numbers;
            loop {
                let mut next_iter = Vec::new();
                for (index, right_number) in current_iter[1..].iter().enumerate() {
                    let left_number = current_iter[index];
                    next_iter.push(*right_number - left_number);
                }
                last_digits.push(*next_iter.last().expect("No last element!"));
                if next_iter.iter().all(|i| *i == 0) {
                    break;
                }
                //println!("{:?}", next_iter);
                current_iter = next_iter;
            }

            last_digits.reverse();
            results.push(last_digits.iter().sum());
        }

        results.iter().sum()
    }

    pub fn sum_part_2(self, content: &str) -> i64 {
        let mut results = Vec::new();
        for line in content.lines() {
            let numbers = numbers_from_line(line);
            let mut first_digits: Vec<i64> = Vec::new();
            first_digits.push(numbers[0]);
            let mut current_iter = numbers;
            loop {
                let mut next_iter = Vec::new();
                for (index, right_number) in current_iter[1..].iter().enumerate() {
                    let left_number = current_iter[index];
                    next_iter.push(*right_number - left_number);
                }
                first_digits.push(*next_iter.first().expect("No first element!"));
                if next_iter.iter().all(|i| *i == 0) {
                    break;
                }
                current_iter = next_iter;
            }

            first_digits.reverse();
            let mut total = 0;
            for d in first_digits {
                total = d - total;
            }
            results.push(total);
        }

        results.iter().sum()
    }
}

fn numbers_from_line(line: &str) -> Vec<i64> {
    let re = Regex::new(r"(-?\d+)").expect("unable to create regex");
    let numbers: Vec<i64> = re
        .find_iter(line)
        .map(|m| m.as_str().parse().expect("unable to parse number"))
        .collect();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let sut = Day9 {};
        assert_eq!(114, sut.sum_part_1(input));
    }
    #[test]
    fn correct_data_parsed() {
        let input = "-1 1";
        assert_eq!(vec![-1, 1], numbers_from_line(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let sut = Day9 {};
        assert_eq!(2, sut.sum_part_2(input));
    }
}

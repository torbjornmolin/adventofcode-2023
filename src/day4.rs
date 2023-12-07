use core::num;
use std::{fs};

use regex::Regex;

use crate::utils::Utils;

#[derive(Clone, Copy)]
pub struct Day4;

impl Day4 {
    const PATH: &'static str = "input/day4.txt";
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
        let mut result = 0;
        let lines = content.split("\n");

        for line in lines {
            let parts: Vec<&str> = line.split(":").collect();
            let score_parts = parts[1];

            let matches: u32 = get_matches(score_parts);

            if matches != 0 {
                result += 2_u32.pow(matches -1);
            }
        }

        result
    }

    pub fn sum_part_2(self, content: &str) -> u32 {
        let lines: Vec<&str> = content.split("\n").collect();
        let mut number_of_cards: Vec<u32> = content.split("\n").map(|_| 1).collect();

        for (index, line) in lines.iter().enumerate() {
            let current_num_cards = number_of_cards[index];

            let parts: Vec<&str> = line.split(":").collect();
            let score_parts = parts[1];

            let score = get_matches(score_parts);

            let start = index +1;
            let usize_score: usize = score.try_into().expect("Could not convert u32 to usize");
            let end = start + usize_score;
            
            for n in start..end {
                number_of_cards[n] += current_num_cards;
            }
        }

        number_of_cards.iter().map(|i| i).sum()
        }
    }


fn get_matches(score_parts: &str) -> u32 {
    let mut result = 0;
    let parts: Vec<&str> = score_parts.split("|").collect();

    let winning = parts[0];
    let my = parts[1];

    let re = Regex::new(r"(\d+)").expect("Could not create regex");

    let winning_matches: Vec<&str> = re.find_iter(winning).map(|c| c.as_str()).collect();
    let my_numbers: Vec<&str> = re.find_iter(my).map(|c| c.as_str()).collect();

    for my in &my_numbers {
        for winning in &winning_matches {
            if my == winning {
                result+=1;
            }
        }
    }

    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let sut = Day4 {};
        assert_eq!(13, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let sut = Day4 {};
        assert_eq!(30, sut.sum_part_2(input));
    }
}

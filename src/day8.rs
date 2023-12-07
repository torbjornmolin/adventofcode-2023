use std::fs;

#[derive(Clone, Copy)]
pub struct Day8;

impl Day8 {
    const PATH: &'static str = "input/day8.txt";
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
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let sut = Day8 {};
        assert_eq!(1, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let sut = Day8 {};
        assert_eq!(5905, sut.sum_part_2(input));
    }
}

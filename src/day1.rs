use core::num;
use std::fs;

#[derive(Clone, Copy)]
pub struct Day1;

impl Day1 {
    const PATH: &'static str = "input/day1.txt";
    pub fn run_part_1(&self) {
        let contents = fs::read_to_string(Self::PATH)
        .expect("Should have been able to read the file");
    
        println!("Part 1 result: {}", self.sum_part_1(&contents));
    }
    pub fn run_part_2(&self) {
        let contents = fs::read_to_string(Self::PATH)
        .expect("Should have been able to read the file");
    
        println!("Part 2 result: {}", self.sum_part_2(&contents));
    }

    pub fn sum_part_1(self, content: &str) -> u32 {
        let mut result: u32 = 0;
        
        let lines = content.split("\n");
        for l in lines {
            let line = l.trim();
            let mut numbers = Vec::<u32>::new();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    numbers.push(c.to_digit(10).unwrap());
                }
            }

            let first_number = numbers.first().expect("Could not get first number") * 10;
            let last_number = numbers.last().expect("Could not get last number");
            result += first_number + last_number;
        }
        result
    }
    
    pub fn sum_part_2(self, content: &str) -> u32 {
        let mut result: u32 = 0;
        
        let lines = content.split("\n");
        for l in lines {
            let line = l.trim();
            let mut numbers = Vec::<u32>::new();
            for (index, c) in line.char_indices() {
                
                if c.is_ascii_digit() {
                    numbers.push(c.to_digit(10).unwrap());
                }
                else {
                    match self.check_start_for_numberstring(&line[index..]) {
                        Some(n) => numbers.push(n),
                        None => (),
                    }
                }
            }

            let first_number = numbers.first().expect("Could not get first number") * 10;
            let last_number = numbers.last().expect("Could not get last number");
            result += first_number + last_number;
        }
        result
    }

    pub fn check_start_for_numberstring(self, input: &str) -> Option<u32> {
        if input.starts_with("one")
        {
            return Some(1);
        }
        if input.starts_with("two")
        {
            return Some(2);
        }
        if input.starts_with("three")
        {
            return Some(3);
        }
        if input.starts_with("four")
        {
            return Some(4);
        }
        if input.starts_with("five")
        {
            return Some(5);
        }
        if input.starts_with("six")
        {
            return Some(6);
        }
        if input.starts_with("seven")
        {
            return Some(7);
        }
        if input.starts_with("eight")
        {
            return Some(8);
        }
        if input.starts_with("nine")
        {
            return Some(9);
        }

        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let sut = Day1 {};
        assert_eq!(142, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        
        let input = 
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let sut = Day1 {};
        assert_eq!(281, sut.sum_part_2(input));
    }

    #[test]
    fn check_start_for_numberstring_ok() {
        
        let input = 
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let sut = Day1 {};
        assert_eq!(9, sut.check_start_for_numberstring(&input[4..]).unwrap());
    }
}
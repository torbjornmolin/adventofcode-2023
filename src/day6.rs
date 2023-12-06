use regex::Regex;
use std::fs;

#[derive(Clone, Copy)]
pub struct Day6;

impl Day6 {
    const PATH: &str = "input/day6.txt";
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
        
        let re = Regex::new(r"\d+").expect("Could not create regex.");
        let parts: Vec<&str> = content.split("\n").collect();
        
        let times: Vec<u64> = re.find_iter(parts[0]).map(|m| m.as_str().parse().expect("Could not parse number from regex match")).collect();
        let records: Vec<u64> = re.find_iter(parts[1]).map(|m| m.as_str().parse().expect("Could not parse number from regex match")).collect();
        
        
        let mut result: u64 = 1;
        for (index, time) in times.iter().enumerate() {
            let record = records[index];
            
            let possible_winns = get_number_of_possible_wins(&record, time);

            result *= possible_winns;
        }

        result
    }

    pub fn sum_part_2(self, content: &str) -> u64 {

        let content = content.replace(" ","");


        self.sum_part_1(&content)
    }
}

fn get_number_of_possible_wins(record: &u64, time: &u64) -> u64 {
    
    let mut result = 0;
    for button_press_time in 1..(time-1)  {
        let run_duration = time - button_press_time;
        let distance = run_duration * button_press_time;
        if distance > *record {
            result += 1;
        }
    }
    result
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let sut = Day6 {};
        assert_eq!(288, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let sut = Day6 {};
        assert_eq!(0, sut.sum_part_2(input));
    }
}

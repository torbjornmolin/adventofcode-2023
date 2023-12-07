use regex::{CaptureLocations, Regex};
use std::fs;

#[derive(Clone, Copy)]
pub struct Day2;

impl Day2 {
    const PATH: &'static str = "input/day2.txt";
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

    /// Sum of the game ids of the possible games.

    pub fn sum_part_1(self, content: &str) -> u32 {
        let mut result: u32 = 0;
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        let lines = content.split("\n");

        for l in lines {
            result += self.check_game(l, max_red, max_green, max_blue);
        }
        result
    }
    fn check_game(self, line: &str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
        let mut main_parts = line.split(':');
        let _ = main_parts.next();
        let games_part = main_parts.next().expect("Cubes part not found");

        let game_id = get_game_id(line);
        let cube_sets = games_part.split(';');

        let re = Regex::new(r"(\d*) (red|green|blue)").unwrap();
        for set in cube_sets {
            for draw in set.split(',') {
                let matches = re.captures(draw).expect("Failed to match line");

                let color = &matches[2];
                let quantity: u32 = (&matches[1]).parse().expect("Failed to get numer of cubes");

                // for invalid games just return 0
                match color.trim() {
                    "red" => {
                        if quantity > max_red {
                            return 0;
                        }
                    }
                    "green" => {
                        if quantity > max_green {
                            return 0;
                        }
                    }
                    "blue" => {
                        if quantity > max_blue {
                            return 0;
                        }
                    }
                    _ => panic!("unexpected color"),
                }
            }
        }

        game_id
    }

    fn get_min_cubes_powers(self, line: &str) -> u32 {
        let mut main_parts = line.split(':');
        let _ = main_parts.next();
        let games_part = main_parts.next().expect("Cubes part not found");

        let game_id = get_game_id(line);
        let cube_sets = games_part.split(';');

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        let re = Regex::new(r"(\d*) (red|green|blue)").unwrap();
        for set in cube_sets {
            for draw in set.split(',') {
                let matches = re.captures(draw).expect("Failed to match line");

                let color = &matches[2];
                let quantity: u32 = (&matches[1]).parse().expect("Failed to get numer of cubes");

                // for invalid games just return 0
                match color.trim() {
                    "red" => {
                        if quantity > min_red {
                            min_red = quantity;
                        }
                    }
                    "green" => {
                        if quantity > min_green {
                            min_green = quantity
                        }
                    }
                    "blue" => {
                        if quantity > min_blue {
                            min_blue = quantity;
                        }
                    }
                    _ => panic!("unexpected color"),
                }
            }
        }

        min_red * min_blue * min_green
    }

    pub fn sum_part_2(self, content: &str) -> u32 {
        let mut result: u32 = 0;

        let lines = content.split("\n");

        for l in lines {
            result += self.get_min_cubes_powers(l);
        }
        result
    }
}

fn get_game_id(line: &str) -> u32 {
    let re = Regex::new(r"Game (\d+)").unwrap();

    let matches = re.captures(line).expect("No match for name regex");
    let group = &matches[1];

    group.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let sut = Day2 {};
        assert_eq!(8, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let sut = Day2 {};
        assert_eq!(2286, sut.sum_part_2(input));
    }
}

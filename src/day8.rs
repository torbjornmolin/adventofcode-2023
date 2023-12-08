use std::{
    collections::HashMap,
    fs::{self},
};

use regex::Regex;

#[derive(Debug)]
struct MazeLine {
    left_index: usize,
    right_index: usize,
    position: usize,
    is_part1_final: bool,
    position_string: String,
}

impl MazeLine {
    pub fn new(line: &str, all_lines: &str) -> Self {
        let re = Regex::new(r"(...) = \((...), (...)\)").expect("Unable to create regex");
        let captures = re.captures(line).expect("Unable to match regex");

        let lines = all_lines.lines();
        let position = captures
            .get(1)
            .map(|m| m.as_str())
            .expect("No position matched")
            .to_string();
        let left = captures
            .get(2)
            .map(|m| m.as_str())
            .expect("No left element matched")
            .to_string();
        let right = captures
            .get(3)
            .map(|m| m.as_str())
            .expect("No left element matched")
            .to_string();

        let position_index = &lines
            .clone()
            .enumerate()
            .find(|(_, line)| line.starts_with(position.as_str()))
            .expect("No index found")
            .0;
        let left_index = &lines
            .clone()
            .enumerate()
            .find(|(_, line)| line.starts_with(left.as_str()))
            .expect("No index found")
            .0;
        let right_index = &lines
            .clone()
            .enumerate()
            .find(|(_, line)| line.starts_with(right.as_str()))
            .expect("No index found")
            .0;
        MazeLine {
            left_index: *left_index,
            right_index: *right_index,
            position: *position_index,
            position_string: position.clone(),
            is_part1_final: &position == "ZZZ",
        }
    }
}

struct Instructions {
    list: Vec<char>,
    index: usize,
    fetch_count: u64,
}

impl Instructions {
    pub fn fetch_next(&mut self) -> char {
        let result = self.list[self.index];
        self.fetch_count += 1;

        self.index += 1;
        if self.index > self.list.len() - 1 {
            // println!(" At end");
            self.index = 0;
        }
        result
    }
    pub fn total_fetches(&self) -> u64 {
        self.fetch_count
    }
    pub fn new(chars: &str) -> Self {
        let mut list = Vec::new();
        for c in chars.chars() {
            match c {
                'L' => list.push('L'),
                'R' => list.push('R'),
                _ => (),
            }
        }
        Instructions {
            list,
            index: 0,
            fetch_count: 0,
        }
    }
}

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
        let parts: Vec<&str> = content.split("\n\n").collect();

        let mut instructions = Instructions::new(parts[0]);
        let maze_list = get_maze_list(parts[1]);
        let maze = get_maze_map(&maze_list);

        let mut maze_line = maze_list
            .iter()
            .find(|m| m.position_string == "AAA")
            .expect("No first element found");
        loop {
            let instruction = instructions.fetch_next();
            let mut next_index: Option<usize> = None;
            match instruction {
                'L' => {
                    next_index = Some(maze_line.left_index);
                }
                'R' => {
                    next_index = Some(maze_line.right_index);
                }
                _ => (),
            }

            let next_index = next_index.expect("Next index not set");
            maze_line = maze[&next_index];

            if maze_line.is_part1_final {
                break;
            }
        }

        instructions.total_fetches()
    }

    pub fn sum_part_2(self, content: &str) -> u64 {
        let parts: Vec<&str> = content.split("\n\n").collect();

        let maze_list = get_maze_list(parts[1]);
        let maze = get_maze_map(&maze_list);

        let start_points: Vec<&MazeLine> = maze_list
            .iter()
            .filter(|maze_line| maze_line.position_string.ends_with("A"))
            .collect();

        let mut endings: Vec<u64> = Vec::new();
        for mut maze_line in start_points {
            let mut instructions = Instructions::new(parts[0].clone());
            loop {
                let instruction = instructions.fetch_next();
                let mut next_index: Option<usize> = None;
                match instruction {
                    'L' => {
                        next_index = Some(maze_line.left_index);
                    }
                    'R' => {
                        next_index = Some(maze_line.right_index);
                    }
                    _ => (),
                }

                let next_index = next_index.expect("Next index not set");
                maze_line = maze[&next_index];

                if maze_line.position_string.ends_with("Z") {
                    println!("Instructions: {}", instructions.fetch_count);
                    endings.push(instructions.fetch_count);
                    break;
                }
            }
        }
        let mut lcm: u64 = endings[0];
        for index in 1..endings.len() {
            lcm = num::integer::lcm(lcm, endings[index]);
        }
        lcm
    }
}

fn get_maze_map(maze_list: &Vec<MazeLine>) -> HashMap<usize, &MazeLine> {
    let mut maze: HashMap<usize, &MazeLine> = HashMap::new();
    for maze_line in maze_list.iter() {
        maze.insert(maze_line.position, maze_line);
    }
    maze
}

fn get_maze_list(lines: &str) -> Vec<MazeLine> {
    let mut maze_list: Vec<MazeLine> = Vec::new();
    let lines_clone = lines.clone();
    for l in lines.lines() {
        let maze_line = MazeLine::new(l, lines_clone);
        maze_list.push(maze_line);
    }
    maze_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let sut = Day8 {};
        assert_eq!(2, sut.sum_part_1(input));
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn correct_part1_example_data_2() {
            let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

            let sut = Day8 {};
            assert_eq!(6, sut.sum_part_1(input));
        }
        #[test]
        fn instruction_list_returns_correct_instruction() {
            let mut sut = Instructions::new("LRL");
            assert_eq!('L', sut.fetch_next());
            assert_eq!('R', sut.fetch_next());
            assert_eq!('L', sut.fetch_next());
            assert_eq!('L', sut.fetch_next());
            assert_eq!('R', sut.fetch_next());
            assert_eq!('L', sut.fetch_next());
        }
    }
}

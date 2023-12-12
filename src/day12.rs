use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

use colored::Colorize;

#[derive(Clone, Copy)]
pub struct Day12;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Pair {
    left: Point,
    right: Point,
    distance: usize,
}

impl Day12 {
    const PATH: &'static str = "input/day12.txt";
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
        let expanded: String = expand(content);
        let galaxies: Vec<Point> = get_galaxies(expanded);
        let mut pairs: Vec<Pair> = get_galaxy_pair(galaxies);
        let updated_pairs = manhattan_distance(&mut pairs);

        updated_pairs.iter().map(|p| p.distance as i64).sum()
    }

    pub fn sum_part_2(self, content: &str) -> i64 {
        let content = String::from(content);
        let galaxies: Vec<Point> = get_galaxies(content.clone());
        let mut pairs: Vec<Pair> = get_galaxy_pair(galaxies);
        let rows_and_cols: (Vec<usize>, Vec<usize>) = get_double_rows_and_cols(&content);
        let updated_pairs = manhattan_distance_2(&mut pairs, rows_and_cols, 1000000 - 1);

        updated_pairs.iter().map(|p| p.distance as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use regex::NoExpand;

    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let sut = Day12 {};
        assert_eq!(374, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "0 3 6 12 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let sut = Day12 {};
        assert_eq!(2, sut.sum_part_2(input));
    }
}

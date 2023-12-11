use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

use colored::Colorize;

#[derive(Clone, Copy)]
pub struct Day11;

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

fn manhattan_distance_2(
    pairs: &mut [Pair],
    rows_and_cols: (Vec<usize>, Vec<usize>),
    gap_value: i32,
) -> &[Pair] {
    for pair in pairs.as_mut() {
        let (rows, cols) = &rows_and_cols;

        let x1 = pair.left.x as i32;
        let x2 = pair.right.x as i32;

        let from_x = min(x1, x2);
        let to_x = max(x1, x2);
        let mut x_gaps = 0;
        for x in from_x..to_x {
            if cols.contains(&(x as usize)) {
                x_gaps += 1;
            }
        }

        let x_distance = (x1 - x2).abs() + x_gaps * gap_value;

        let y1 = pair.left.y as i32;
        let y2 = pair.right.y as i32;

        let from_y = min(y1, y2);
        let to_y = max(y1, y2);

        let mut y_gaps = 0;
        for y in from_y..to_y {
            if rows.contains(&(y as usize)) {
                y_gaps += 1;
            }
        }
        let y_distance = (y1 - y2).abs() + y_gaps * gap_value;

        let distance = x_distance + y_distance;
        pair.distance = distance as usize;
    }
    pairs
}

fn get_double_rows_and_cols(content: &str) -> (Vec<usize>, Vec<usize>) {
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    let mut has_galaxie = HashMap::new();

    for (line_index, line) in content.lines().enumerate() {
        let mut is_blank = true;
        for (char_index, c) in line.char_indices() {
            if c == '#' {
                is_blank = false;
                has_galaxie.insert(char_index, true);
            }
        }
        if is_blank {
            rows.push(line_index);
        }
    }
    for (index, _) in content.lines().enumerate() {
        if !has_galaxie.contains_key(&index) {
            cols.push(index);
        }
    }

    (rows, cols)
}

fn manhattan_distance(pairs: &mut [Pair]) -> &[Pair] {
    for pair in pairs.as_mut() {
        let x1 = pair.left.x as i32;
        let x2 = pair.right.x as i32;

        let y1 = pair.left.y as i32;
        let y2 = pair.right.y as i32;
        let distance = (x1 - x2).abs() + (y1 - y2).abs();
        pair.distance = distance as usize;
    }
    pairs
}

fn get_galaxy_pair(galaxies: Vec<Point>) -> Vec<Pair> {
    let mut result = Vec::new();
    for start in 0..galaxies.len() {
        let left = galaxies[start].clone();
        for index in start + 1..galaxies.len() {
            result.push(Pair {
                left,
                right: galaxies[index].clone(),
                distance: 0,
            });
        }
    }

    result
}

fn get_galaxies(expanded: String) -> Vec<Point> {
    let mut result = Vec::new();
    for (y, line) in expanded.lines().enumerate() {
        for (x, char) in line.char_indices() {
            if char == '#' {
                result.push(Point { x, y });
            }
        }
    }
    result
}

fn expand(content: &str) -> String {
    let mut string_vec: Vec<String> = content.lines().map(|m| String::from(m)).collect();
    let mut has_galaxie = HashMap::new();

    let mut insert_count = 0;
    for (index, line) in content.lines().enumerate() {
        let mut is_blank = true;
        for (char_index, c) in line.char_indices() {
            if c == '#' {
                is_blank = false;
                has_galaxie.insert(char_index, true);
            }
        }
        if is_blank {
            let new_string_length = line.len();
            let new_string = std::iter::repeat(".")
                .take(new_string_length)
                .collect::<String>();
            string_vec.insert(index + insert_count, new_string);

            insert_count += 1;
        }
    }

    let initial_string_length = string_vec[0].len();
    for line_index in 0..string_vec.len() {
        let mut number_of_inserts = 0;
        for char_index in 0..initial_string_length {
            if !has_galaxie.contains_key(&char_index) {
                string_vec[line_index].insert_str(char_index + number_of_inserts, ".");
                number_of_inserts += 1;
            }
        }
    }

    let mut result = String::new();
    for l in string_vec {
        let s = format!("{}\n", l);
        result.push_str(s.as_str());
    }
    result
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

        let sut = Day11 {};
        assert_eq!(374, sut.sum_part_1(input));
    }
    #[test]
    fn correct_get_distance_5_to_9() {
        let mut pair = Pair {
            left: Point { x: 1, y: 0 },
            right: Point { x: 5, y: 5 },
            distance: 0,
        };

        let mut vec = vec![pair];
        let new = manhattan_distance(&mut vec);
        assert_eq!(9, new[0].distance);
    }
    #[test]
    fn correct_get_distance_1_to_9() {
        let pair = Pair {
            left: Point { x: 4, y: 0 },
            right: Point { x: 9, y: 10 },
            distance: 0,
        };

        let mut vec = vec![pair];
        let new = manhattan_distance(&mut vec);
        assert_eq!(15, new[0].distance);
    }
    #[test]
    fn correct_get_distance_horizontal() {
        let pair = Pair {
            left: Point { x: 1, y: 0 },
            right: Point { x: 3, y: 0 },
            distance: 0,
        };

        let mut vec = vec![pair];
        let new = manhattan_distance(&mut vec);
        assert_eq!(2, new[0].distance);
    }
    #[test]
    fn correct_get_distance_vertical() {
        let pair = Pair {
            left: Point { x: 1, y: 0 },
            right: Point { x: 1, y: 2 },
            distance: 0,
        };

        let mut vec = vec![pair];
        let new = manhattan_distance(&mut vec);
        assert_eq!(2, new[0].distance);
    }
    #[test]
    fn correct_get_distance_3_to_6() {
        let pair = Pair {
            left: Point { x: 0, y: 2 },
            right: Point { x: 12, y: 7 },
            distance: 0,
        };

        let mut vec = vec![pair];
        let new = manhattan_distance(&mut vec);
        assert_eq!(17, new[0].distance);
    }
    #[test]
    fn expand_correct_cols() {
        let input = "##.##
##.##";
        let expected = "##..##
##..##
";

        let result = expand(input);
        assert_eq!(expected, result);
    }
    #[test]
    fn expand_correct_rows() {
        let input = "#####
.....
#####
";
        let expected = "#####
.....
.....
#####
";

        let result = expand(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn expand_correct_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

        let result = expand(input);
        assert_eq!(expected, result);
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

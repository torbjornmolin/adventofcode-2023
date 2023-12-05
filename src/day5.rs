use regex::Regex;
use std::fs;

#[derive(Clone, Copy)]
pub struct Day5;

struct RangeMap {
    from_start: u64,
    to_start: u64,
    length: u64,
}

impl RangeMap {
    pub fn do_mapping(&self, input: u64) -> Option<u64> {
        if input >= self.from_start && input <= self.from_start + self.length - 1 {
            let offset = input - self.from_start;

            return Some(self.to_start + offset);
        }
        None
    }
}

impl Day5 {
    const PATH: &str = "input/day5.txt";
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
        let mut result = 0;

        let parts: Vec<&str> = content.split("\n\n").collect();

        let seeds: Vec<u64> = get_seed_numbers(parts[0]);

        let mut maps: Vec<Vec<RangeMap>> = Vec::new();
        get_all_mappings(parts, &mut maps);

        for seed in seeds {
            let mut mapped_seed_no = seed;
            for map in &maps {
                for mapping in map {
                    let mapped = mapping.do_mapping(mapped_seed_no);
                    match mapped {
                        Some(mapped_number) => {
                            mapped_seed_no = mapped_number;
                            break;
                        }

                        None => (),
                    }
                }
            }
            if result == 0 || mapped_seed_no < result {
                result = mapped_seed_no;
            }
            println!("Mapped seed {} to {}", seed, mapped_seed_no);
        }

        result
    }

    pub fn sum_part_2(self, content: &str) -> u64 {
        let mut result = 0;

        let parts: Vec<&str> = content.split("\n\n").collect();

        let seed_ranges: Vec<(u64, u64)> = get_seed_number_ranges(parts[0]);

        let mut maps: Vec<Vec<RangeMap>> = Vec::new();
        get_all_mappings(parts, &mut maps);

        for seed_range in seed_ranges {
            for seed in seed_range.0..(seed_range.0 + seed_range.1 - 1) {
                let mut mapped_seed_no = seed;
                for map in &maps {
                    for mapping in map {
                        let mapped = mapping.do_mapping(mapped_seed_no);
                        match mapped {
                            Some(mapped_number) => {
                                mapped_seed_no = mapped_number;
                                break;
                            }

                            None => (),
                        }
                    }
                }
                if result == 0 || mapped_seed_no < result {
                    result = mapped_seed_no;
                }
                //println!("Mapped seed {} to {}", seed, mapped_seed_no);
            }
        }

        result
    }
}

fn get_all_mappings(parts: Vec<&str>, maps: &mut Vec<Vec<RangeMap>>) {
    for part in parts.iter().skip(1) {
        let map = get_mappings(part);

        maps.push(map);
    }
}

fn get_mappings(part: &str) -> Vec<RangeMap> {
    let mut result = Vec::new();

    let re = Regex::new(r"\d+").expect("Could not create regex.");
    for line in part.lines().skip(1) {
        let numbers: Vec<u64> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().expect("Could not parse seed number"))
            .collect();
        let mapping = RangeMap {
            to_start: numbers[0],
            from_start: numbers[1],
            length: numbers[2],
        };

        result.push(mapping);
    }
    result
}

fn get_seed_numbers(seed_line: &str) -> Vec<u64> {
    let re = Regex::new(r"\d+").expect("Could not create regex.");
    let seeds = re
        .find_iter(seed_line)
        .map(|m| m.as_str().parse().expect("Could not parse seed number"))
        .collect();

    seeds
}

fn get_seed_number_ranges(seed_line: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"\d+").expect("Could not create regex.");
    let numbers: Vec<u64> = re
        .find_iter(seed_line)
        .map(|m| m.as_str().parse().expect("Could not parse seed number"))
        .collect();

    let mut seeds = Vec::new();

    for c in numbers.chunks(2) {
        seeds.push((c[0], c[1]));
    }
    seeds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_example_data() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let sut = Day5 {};
        assert_eq!(35, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let sut = Day5 {};
        assert_eq!(46, sut.sum_part_2(input));
    }
}

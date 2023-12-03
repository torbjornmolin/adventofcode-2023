pub struct Utils;
impl Utils {
    pub fn text_to_2d_array(input: &str) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();

        let mut current_vec = Vec::<char>::new();
        for c in input.chars() {
            if c == '\n' {
                result.push(current_vec);
                current_vec = Vec::<char>::new();
                continue;
            }
            current_vec.push(c.clone());
        }
        if current_vec.len() > 0 {
            result.push(current_vec);
        }

        result
    }

    pub fn get_numer_at_coordinate(map: &Vec<Vec<char>>, line_no: usize, char_index: usize) -> u32 {
        let line = &map[line_no];
        let mut index = char_index;
        while index > 0 && line[index].is_digit(10) {
            index -= 1;
        }
        if !line[index].is_digit(10) {
            index += 1;
        }
        let start = index;

        while index < line.len() && line[index].is_digit(10) {
            index += 1;
        }
        let end = index;

        let str: String = line[start..end].into_iter().collect();
        let result: u32 = str.parse().expect("Unable to parse number");
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_vec_from_input() {
        let input = "467.
...*
";

        let mut expected: Vec<Vec<char>> = Vec::<Vec<char>>::new();
        expected.push(vec!['4', '6', '7', '.']);
        expected.push(vec!['.', '.', '.', '*']);

        assert_eq!(expected, Utils::text_to_2d_array(input));
    }

    #[test]
    fn get_number_at_coordinate_correct_for_middle() {
        let input = "467..114..
...*......
..35...633
.......#..
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let map = Utils::text_to_2d_array(input);
        assert_eq!(114, Utils::get_numer_at_coordinate(&map, 0, 6));
    }
}

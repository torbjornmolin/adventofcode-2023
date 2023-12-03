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
}

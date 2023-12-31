use regex::Regex;
use core::num;
use std::{fs, collections::HashMap};


#[derive(Clone, Debug)]
pub struct Hand {
    bid: u32,
    initial_cards: String,
    mapped_cards: String,
    hand_sort_string: String
}

#[derive(Clone, Copy)]
pub struct Day7;

impl Day7 {
    const PATH: &'static str = "input/day7.txt";
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
        let re = Regex::new(r"(.{5}) (\d*)").expect("Could not create regex.");

        let mut hands = Vec::<Hand>::new();
        for line in content.lines() {
            let captures = re.captures(line).expect("Unable to match regex");

            let cards = captures.get(1).map(|m| m.as_str()).expect("No cards matched").to_string();
            let bid: u32 = captures.get(2).map(|m| m.as_str()).expect("No cards matched").parse().expect("Unable to parse bid");

            let mapped_cards: String = map_cards(&cards);
            let hand_strength: char = get_hand_strength(&cards);
            let hand_sort_string = format!("{}_{}", hand_strength, mapped_cards);
            hands.push(Hand {
                bid,
                initial_cards: cards,
                mapped_cards,
                hand_sort_string
            });
        }
        hands.sort_by_key(|hand| hand.hand_sort_string.clone());
        hands.reverse();
        
        let mut result:u64 = 0;
        for (index, hand) in hands.iter().enumerate() {
            result += (index +1) as u64 * hand.bid as u64;
        }

        result
    }

    pub fn sum_part_2(self, content: &str) -> u64 {
        let re = Regex::new(r"(.{5}) (\d*)").expect("Could not create regex.");

        let mut hands = Vec::<Hand>::new();
        for line in content.lines() {
            let captures = re.captures(line).expect("Unable to match regex");

            let cards = captures.get(1).map(|m| m.as_str()).expect("No cards matched").to_string();
            let bid: u32 = captures.get(2).map(|m| m.as_str()).expect("No cards matched").parse().expect("Unable to parse bid");

            let mapped_cards: String = map_cards_part2(&cards);
            let hand_strength: char = get_hand_strength_part2(&cards);
            let hand_sort_string = format!("{}_{}", hand_strength, mapped_cards);
            hands.push(Hand {
                bid,
                initial_cards: cards,
                mapped_cards,
                hand_sort_string
            });
        }
        hands.sort_by_key(|hand| hand.hand_sort_string.clone());
        hands.reverse();
        
        let mut result:u64 = 0;
        for (index, hand) in hands.iter().enumerate() {
            result += (index +1) as u64 * hand.bid as u64;
        }

        result
    }
}

fn get_hand_strength_part2(cards: &str) -> char {
    let non_joker_cards: Vec<char> = cards.chars().filter(|c| *c!='J').collect();
    let joker_cards: Vec<char> = cards.chars().filter(|c| *c=='J').collect();
    // Five of a kind
    if is_five_of_a_kind_w_jokers(&non_joker_cards, joker_cards.len()) {
        return '0';
    }
    // Four of a kind
    else if is_four_of_a_kind_w_jokers(&non_joker_cards, joker_cards.len()) {
        return '1';
    }
    // Full house
    else if is_full_house_w_jokers(&non_joker_cards, joker_cards.len()) {
        return '2';
    }
    // Three of a kind

    else if is_three_of_a_kind_w_jokers(&non_joker_cards, joker_cards.len()) {
        return '3';
    }
    // Two pair
    else if is_two_pair_w_jokers(&non_joker_cards, joker_cards.len()) {
        return '4';
    }
    // One pair
    else if is_one_pair_w_jokers(&non_joker_cards, joker_cards.len()) {
        return '5';
    }
    // High card
    else {
        return '6';
    }
}

fn is_five_of_a_kind_w_jokers(non_joker_cards: &Vec<char>, number_of_jokers: usize) -> bool {
    if number_of_jokers == 5 {
        return true;
    }
    else {
        let first = non_joker_cards.first().expect("Unable to get first card");
        return non_joker_cards.iter().all(|c| c==first);
    }
}

fn is_four_of_a_kind_w_jokers(non_joker_cards: &Vec<char>, number_of_jokers: usize) -> bool {
    let non_joker_cards_string = non_joker_cards.iter().collect::<String>();
    let card_counts = get_card_counts(non_joker_cards_string.as_str());

    *card_counts.values().max().expect("Unable to get value max") + number_of_jokers as i32 == 4
}
fn is_full_house_w_jokers(non_joker_cards: &[char], number_of_jokers: usize) -> bool {
    // at this point we can only have MAX 2 jokers, otherwise we would have matched four of a kind.

    let non_joker_cards_string = non_joker_cards.iter().collect::<String>();
    let card_counts = get_card_counts(non_joker_cards_string.as_str());
    
    let mut card_counts: Vec<i32> = card_counts.values().map(|v| *v).collect();
    card_counts.sort_unstable();
    card_counts.reverse();

    if card_counts.len() > 1 && card_counts[0] + card_counts[1] + number_of_jokers as i32 == 5 {
        return true;
    }
    else if card_counts.len() == 1 {
        return true;
    }
    else {
        return false;
    }
    
}
fn is_three_of_a_kind_w_jokers(non_joker_cards: &[char], number_of_jokers: usize) -> bool {
    let non_joker_cards_string = non_joker_cards.iter().collect::<String>();
    let card_counts = get_card_counts(non_joker_cards_string.as_str());
    
    let mut card_counts: Vec<i32> = card_counts.values().map(|v| *v).collect();
    card_counts.sort_unstable();
    card_counts.reverse();

    if card_counts.len() >= 1 && card_counts[0] + number_of_jokers as i32 == 3 {
        return true;
    }
    else {
        return false;
    }
}

fn is_two_pair_w_jokers(non_joker_cards: &[char], number_of_jokers: usize) -> bool {
    let non_joker_cards_string = non_joker_cards.iter().collect::<String>();
    let card_counts = get_card_counts(non_joker_cards_string.as_str());
    
    let mut card_counts: Vec<i32> = card_counts.values().map(|v| *v).collect();
    card_counts.sort_unstable();
    card_counts.reverse();

    if card_counts.len() > 1 && card_counts[0] + card_counts[1] + number_of_jokers as i32 == 4 {
        return true;
    }
    else {
        return false;
    }
}

fn is_one_pair_w_jokers(non_joker_cards: &[char], number_of_jokers: usize) -> bool {
    let non_joker_cards_string = non_joker_cards.iter().collect::<String>();
    let card_counts = get_card_counts(non_joker_cards_string.as_str());
    
    let mut card_counts: Vec<i32> = card_counts.values().map(|v| *v).collect();
    card_counts.sort_unstable();
    card_counts.reverse();

    if card_counts.len() >= 1 && card_counts[0]  + number_of_jokers as i32 == 2 {
        return true;
    }
    else {
        return false;
    }
}
fn map_cards_part2(cards: &str) -> String {
    let mut mapped_cards = Vec::<char>::new();
    for c in cards.chars() {
        match c {
            'A' => mapped_cards.push('A'),
            'K' => mapped_cards.push('B'),
            'Q' => mapped_cards.push('C'),
            'T' => mapped_cards.push('E'),
            '9' => mapped_cards.push('F'),
            '8' => mapped_cards.push('G'),
            '7' => mapped_cards.push('H'),
            '6' => mapped_cards.push('I'),
            '5' => mapped_cards.push('J'),
            '4' => mapped_cards.push('K'),
            '3' => mapped_cards.push('L'),
            '2' => mapped_cards.push('M'),
            '1' => mapped_cards.push('N'),
            'J' => mapped_cards.push('O'),
            _ => panic!("Unexpectec card")
        }
    }

    mapped_cards.iter().collect()
}

fn get_hand_strength(cards: &str) -> char {
    // Five of a kind
    if is_five_of_a_kind(cards) {
        return '0';
    }
    // Four of a kind
    else if is_four_of_a_kind(cards) {
        return '1';
    }
    // Full house
    else if is_full_house(cards) {
        return '2';
    }
    // Three of a kind

    else if is_three_of_a_kind(cards) {
        return '3';
    }
    // Two pair
    else if is_two_pair(cards) {
        return '4';
    }
    // One pair
    else if is_one_pair(cards) {
        return '5';
    }
    // High card
    else {
        return '6';
    }
}

fn is_one_pair(mapped_sorted_cards: &str) -> bool {
    let card_counts = get_card_counts(mapped_sorted_cards);

    let mut values: Vec<i32> = card_counts.values().map(|v| *v).collect();
    values.sort_unstable();
    values.reverse();


    values.len() > 1 && values[0] == 2
}

fn is_two_pair(mapped_sorted_cards: &str) -> bool {
    let card_counts = get_card_counts(mapped_sorted_cards);

    let mut values: Vec<i32> = card_counts.values().map(|v| *v).collect();
    values.sort_unstable();
    values.reverse();


    values.len() == 3 && values[0] == 2 && values[1] == 2
}

fn is_three_of_a_kind(mapped_sorted_cards: &str) -> bool {
    let card_counts = get_card_counts(mapped_sorted_cards);

    let mut values: Vec<i32> = card_counts.values().map(|v| *v).collect();
    values.sort_unstable();
    values.reverse();

    values.len() > 1 && values[0] == 3
}

fn is_full_house(mapped_sorted_cards: &str) -> bool {
    let card_counts = get_card_counts(mapped_sorted_cards);

    let mut values: Vec<i32> = card_counts.values().map(|v| *v).collect();
    values.sort_unstable();
    values.reverse();

    values.len() == 2 && values[0] == 3 && values[1] == 2
}

fn is_four_of_a_kind(mapped_sorted_cards: &str) -> bool {
    let card_counts = get_card_counts(mapped_sorted_cards);

     *card_counts.values().max().expect("Unable to get value max") == 4
}

fn is_five_of_a_kind(mapped_sorted_cards: &str) -> bool {
    let first = mapped_sorted_cards.chars().nth(0).expect("no elements");
    return mapped_sorted_cards.chars().all(|c| c== first);
}
fn get_card_counts(mapped_sorted_cards: &str) -> HashMap<char, i32> {
    let mut card_counts = HashMap::new();

    for card in mapped_sorted_cards.chars() {
        if card_counts.contains_key(&card) {
            *card_counts.get_mut(&card).expect("unable to get value from hashmap") += 1;
        }
        else {
            card_counts.insert(card, 1);
        }
    }
    card_counts
}

fn map_cards(cards: &str) -> String {
    let mut mapped_cards = Vec::<char>::new();
    for c in cards.chars() {
        match c {
            'A' => mapped_cards.push('A'),
            'K' => mapped_cards.push('B'),
            'Q' => mapped_cards.push('C'),
            'J' => mapped_cards.push('D'),
            'T' => mapped_cards.push('E'),
            '9' => mapped_cards.push('F'),
            '8' => mapped_cards.push('G'),
            '7' => mapped_cards.push('H'),
            '6' => mapped_cards.push('I'),
            '5' => mapped_cards.push('J'),
            '4' => mapped_cards.push('K'),
            '3' => mapped_cards.push('L'),
            '2' => mapped_cards.push('M'),
            '1' => mapped_cards.push('N'),
            _ => panic!("Unexpectec card")
        }
    }

    mapped_cards.iter().collect()
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

        let sut = Day7 {};
        assert_eq!(6440, sut.sum_part_1(input));
    }

    #[test]
    fn correct_part2_example_data() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let sut = Day7 {};
        assert_eq!(5905, sut.sum_part_2(input));
    }
}

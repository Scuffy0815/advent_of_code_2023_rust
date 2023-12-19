// each pair is x,y = column, row

use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    actual_numbers: Vec<i32>,
}

impl Card {
    fn card_points(&self) -> (i32, i32) {
        let mut points = 0;
        let mut wins = 0;
        for number in &self.actual_numbers {
            if self.winning_numbers.contains(number) {
                wins += 1;
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        return (points, wins);
    }
}


fn parse_card(line: &String) -> Card {
    let mut parts = line.split(":");
    let id = parts.next().unwrap().split(" ").filter(|s| !s.is_empty()).nth(1).unwrap().parse::<i32>().unwrap();

    let mut numbers = parts.next().unwrap().split("|");
    let winning_numbers = numbers.next().unwrap().split(" ").flat_map(|s| s.parse::<i32>()).collect::<Vec<i32>>();
    let actual_numbers = numbers.next().unwrap().split(" ").flat_map(|s| s.parse::<i32>()).collect::<Vec<i32>>();

    return Card { id, winning_numbers, actual_numbers };
}

pub fn sum_points(input: &Vec<String>) -> i32 {
    let mut points = 0;
    for line in input {
        let card = parse_card(line);
        let (card_points, _) = card.card_points();
        points += card_points;
    }
    return points;
}

pub fn sum_card_copies(input: &Vec<String>) -> i32 {
    let mut cards = 0;
    let mut copies = HashMap::new();
    for line in input {
        let card = parse_card(line);
        let (_, wins) = card.card_points();
        let this_copies = copies.entry(card.id).or_insert(1).clone();

        for n in card.id..card.id + wins {
            let count = copies.entry(n+1).or_insert(1);
            *count += 1 * this_copies;
        }
        cards += this_copies;
    }
    return cards;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_numbers() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();

        let card = parse_card(&line);

        let expected = Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], actual_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        assert_eq!(expected, card);
    }

    #[test]
    fn should_calculate_card_points() {
        let card = Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], actual_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };

        assert_eq!((8, 4), card.card_points());
    }

    #[test]
    fn should_calculate_cards_points() {
        let input: Vec<String> =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(13, sum_points(&input));
    }

    #[test]
    fn should_calculate_card_amount() {
        let input: Vec<String> =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(30, sum_card_copies(&input));
    }
}

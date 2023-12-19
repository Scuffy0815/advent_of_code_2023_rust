use std::collections::HashMap;
use regex::Regex;
use ::phf::{Map, phf_map};

static COLOR_TO_MAX: Map<&str, i32> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};
//not possible.... so I have to rebuild the map on every function call or pass it.... or clojure
// const COLOR_TO_MAX: HashMap<&str, i32> = HashMap::from([
//     ("red", 12),
//     ("green", 13),
//     ("blue", 14)
// ]);

#[derive(PartialEq)]
#[derive(Debug)]
struct RGB {
    game: i32,
    red: i32,
    green: i32,
    blue: i32,
}

fn color_values(line: &str) -> Vec<(i32, &str)> {
    let re = Regex::new(r"(\d+) ([a-z]+)").unwrap();
    return _color_values(line, &re);
}

fn _color_values<'a>(line: &'a str, re: &Regex) -> Vec<(i32, &'a str)> {
    let mut results = vec![];
    for (_, [num, color]) in re.captures_iter(&line).map(|cap| cap.extract()) {
        results.push((num.parse::<i32>().unwrap_or(0), color));
    }
    return results;
}

fn validate_game(x: &Vec<(i32, &str)>) -> bool {
    for (amount, color) in x {
        if amount > COLOR_TO_MAX.get(color).unwrap_or(&0) {
            return false;
        }
    }
    return true;
}

fn power_of_set(x: &Vec<(i32, &str)>) -> i32 {
    let minimum_by_color = x.iter().fold(HashMap::new(), |mut acc, (amount, color)|
        {
            let current = acc.entry(color).or_insert(*amount);
            if amount > current {
                acc.insert(color, *amount);
            }
            return acc;
        });
    let vals: Vec<i32> = minimum_by_color.values().map(|x| *x).collect();
    let power: i32 = vals.iter().fold(1, |a, b| (a * b));

    return power;
}

fn parse_line(line: &String) -> Option<i32> {
    let mut splits = line.split(":");
    let game: i32 = splits.next().unwrap().split(" ").nth(1).unwrap().parse().unwrap();
    let colors = color_values(splits.next().unwrap_or(""));
    if validate_game(&colors) {
        return Some(game);
    }
    return None;
}

fn power_of_line(line: &String) -> i32 {
    let mut splits = line.split(":");
    let colors = color_values(splits.nth(1).unwrap_or(""));
    return power_of_set(&colors);

}
fn power(input: &Vec<String>) -> i32 {
    return input.iter()
        .map(|line| power_of_line(&line))
        .sum();
}

fn sum_solvable_games(input: &Vec<String>) -> i32 {
    return input.iter()
        .flat_map(|line| parse_line(&line))
        .sum();
}

pub fn decode_file() -> i32 {
    return sum_solvable_games(&super::read_aoc_lines("2"));
}

pub fn decode_file_part_2() -> i32 {
    return power(&super::read_aoc_lines("2"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_regex() {
        let input = " 8 green, 6 blue, 5 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let expected = vec![(8, "green"), (6, "blue"), (5, "red"), (5, "blue"), (4, "red"), (13, "green"), (5, "green"), (1, "red")];
        assert_eq!(expected, color_values(&input));
    }

    #[test]
    fn should_validate() {
        let input = vec![(8, "green"), (6, "blue"), (5, "red"), (5, "blue"), (4, "red"), (13, "green"), (5, "green"), (1, "red")];
        let input_false = vec![(8, "green"), (16, "blue"), (5, "red"), (5, "blue"), (4, "red"), (13, "green"), (5, "green"), (1, "red")];
        assert_eq!(true, validate_game(&input));
        assert_eq!(false, validate_game(&input_false));
    }

    #[test]
    fn should_parse_line() {
        let input = "Game 3: 8 green, 6 blue, 5 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string();
        assert_eq!(Some(3), parse_line(&input));
    }

    #[test]
    fn should_not_parse_line_with_max_value_exceeded() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string();
        assert_eq!(None, parse_line(&input));
    }

    #[test]
    fn should_decode_test_file() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(8, sum_solvable_games(&input));
    }

    #[test]
    fn should_calculate_power_test_file() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(2286, power(&input));
    }
}
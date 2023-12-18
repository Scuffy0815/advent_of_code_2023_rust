const NUMBER_STRINGS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn string_to_num(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.chars().next().expect("REASON").to_digit(10).unwrap_or(0)
    }
}

pub fn extract_first_and_last_number(line: &String) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_digit(10));
    let first = numbers.next().and_then(|c| { c.to_digit(10) }).unwrap_or(0);
    let last = numbers.next_back().and_then(|c| { c.to_digit(10) }).unwrap_or(first);
    return first * 10 + last;
}

fn decode_lines(lines: Vec<String>) -> u32 {
    return lines.iter()
        .map(|s| {
            extract_first_and_last_number(s)
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0);
}

fn decode_line(line: &String) -> u32 {
    let mut matches: Vec<_> = [].to_vec();
    for pattern in NUMBER_STRINGS.iter() {
        matches.extend(line.match_indices(pattern));
    }

    matches.sort_by(|a: &(usize, &str), b: &(usize, &str)| { a.0.cmp(&b.0) });

    let first: &(usize, &str) = matches.first().unwrap();
    let last: &(usize, &str) = matches.last().unwrap();
    let result = string_to_num(first.1) * 10 + string_to_num(last.1);
    return result;
}

fn decode_lines_2(lines: Vec<String>) -> u32 {
    lines.iter().map(decode_line).reduce(|a, b| a + b).unwrap_or(0)
}

pub fn decode_file() -> u32 {
    return decode_lines(super::read_aoc_lines("1"));
}

pub fn decode_file_b() -> u32 {
    return decode_lines_2(super::read_aoc_lines("1"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_line() {
        let input = "1abc2".to_string();
        assert_eq!(12, extract_first_and_last_number(&input));
    }

    #[test]
    fn should_match_number_strings() {
        let input = "eightwothree".to_string();
        assert_eq!(83, decode_line(&input));
    }

    #[test]
    fn should_decode_test_file() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        assert_eq!(142, decode_lines(input));
    }

    #[test]
    fn should_decode_test_file_with_number_strings() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(281, decode_lines_2(input));
    }
}

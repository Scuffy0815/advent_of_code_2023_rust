// each pair is x,y = column, row

#[derive(PartialEq)]
#[derive(Debug)]
struct Part {
    number: i32,
    positions: Vec<(i32, i32)>,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Char {
    character: char,
    position: (i32, i32),
}

fn pos_is_adjacent(p0: (i32, i32), p1: (i32, i32)) -> bool {
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    return (x0 - x1).abs() <= 1 && (y0 - y1).abs() <= 1;
}

fn is_adjacent(part: &Part, char: &Char) -> bool {
    for pos in part.positions.iter() {
        if pos_is_adjacent(*pos, char.position) {
            return true;
        }
    }
    return false;
}

fn extract_chars_and_parts(lines: &Vec<String>) -> (Vec<Char>, Vec<Part>) {
    let mut chars = vec![];
    let mut parts = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut number: i32 = 0;
        let mut positions = vec![];
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number = number * 10 + c.to_digit(10).unwrap() as i32;
                positions.push((x as i32, y as i32));

            } else if !c.is_digit(10) {
                if c != '.' {
                    let char = Char { character: c, position: (x as i32, y as i32) };
                    chars.push(char);
                }

                if number > 0 {
                    let part = Part { number, positions };
                    parts.push(part);
                    number = 0;
                    positions = vec![];
                }
            }
        }
        if number > 0 {
            let part = Part { number, positions };
            parts.push(part);
        }

    }
    return (chars, parts);
}

fn part_has_any_char(part: &Part, chars: &Vec<Char>) -> bool {
    for char in chars.iter() {
        if is_adjacent(part, char) {
            return true;
        }
    }
    return false;
}

fn gear_ration(char: &Char, parts: &Vec<Part>) -> i32 {
    if char.character == '*' {
        let gear_parts: Vec<&Part> = parts.iter().filter(|part| is_adjacent(part, &char)).collect();
        if gear_parts.len() == 2 {
            return gear_parts.iter().fold(1, |acc, part| acc * part.number);
        }
    }
    return  0;
}

pub fn calc_gear_ratio(input: &Vec<String>) -> i32 {
    let (chars, parts) = extract_chars_and_parts(input);
    let part_sum = chars.iter()
        .map(|char| gear_ration(char, &parts))
        .sum();
    return part_sum;
}

pub fn check_parts(input: &Vec<String>) -> i32 {
    let (chars, parts) = extract_chars_and_parts(input);
    let part_sum = parts.iter()
        .filter(|part| part_has_any_char(part, &chars))
        .map(|part| part.number)
        .sum();

    return part_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_adjacent() {
        let part = Part { number: 1, positions: vec![(42, 55), (43, 55)] };
        let chars_adjacent = vec![
            Char { character: 'a', position: (41, 54) },
            Char { character: 'a', position: (41, 55) },
            Char { character: 'a', position: (41, 56) },
            Char { character: 'a', position: (42, 54) },
            Char { character: 'a', position: (42, 56) },
            Char { character: 'a', position: (43, 54) },
            Char { character: 'a', position: (43, 56) },
            Char { character: 'a', position: (44, 54) },
            Char { character: 'a', position: (44, 55) },
            Char { character: 'a', position: (44, 56) },
        ];
        let chars_not_adjacent = vec![
            Char { character: 'a', position: (42, 66) },
            Char { character: 'a', position: (41, 52) },
            Char { character: 'a', position: (55, 52) },
            Char { character: 'a', position: (56, 52) },
        ];

        for char in chars_adjacent {
            assert!(is_adjacent(&part, &char));
        }
        for char in chars_not_adjacent {
            assert!(!is_adjacent(&part, &char));
        }
    }


    #[test]
    fn should_extract_parts_and_chars() {
        let input: Vec<String>=
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string().split("\n").map(|s| s.to_string()).collect();

        let expected_parts = vec![
            Part { number: 467, positions: vec![(0, 0), (1, 0), (2, 0)] },
            Part { number: 114, positions: vec![(5, 0), (6, 0), (7,0)] },
            Part { number: 35, positions: vec![(2, 2), (3, 2)] },
            Part { number: 633, positions: vec![(6, 2), (7, 2), (8, 2)] },
            Part { number: 617, positions: vec![(0, 4), (1, 4), (2, 4)] },
            Part { number: 58, positions: vec![(7, 5), (8, 5)] },
            Part { number: 592, positions: vec![(2, 6), (3, 6), (4, 6)] },
            Part { number: 755, positions: vec![(6, 7), (7, 7), (8, 7)] },
            Part { number: 664, positions: vec![(1, 9), (2, 9), (3, 9)] },
            Part { number: 598, positions: vec![(5, 9), (6, 9), (7, 9)] },
        ];
        let expected_chars = vec![
            Char { character: '*', position: (3, 1) },
            Char { character: '#', position: (6, 3) },
            Char { character: '*', position: (3, 4) },
            Char { character: '+', position: (5, 5) },
            Char { character: '$', position: (3, 8) },
            Char { character: '*', position: (5, 8) },
        ];

        // when
        let (chars, parts) = extract_chars_and_parts(&input);

        // then
        assert_eq!(expected_chars, chars);
        assert_eq!(expected_parts, parts);
    }
    #[test]
    fn should_calculate_power_test_file() {
        let input: Vec<String>=
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(4361, check_parts(&input));
    }

    #[test]
    fn should_calculate_gear_value() {
        let input: Vec<String>=
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(467835, calc_gear_ratio(&input));
    }
}
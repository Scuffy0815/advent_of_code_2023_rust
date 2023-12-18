use std::fs;

pub fn read_aoc_lines(day: &str) -> Vec<String> {
    let filename = format!("challenge_inputs/day{}.txt", day);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    return lines;
}

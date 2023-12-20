pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod util;
use aoc_23::util::io::read_aoc_lines;

fn main() {

    // println!("{}", day1::decode_file_b());
    // println!("day 2 a: {}", day2::decode_file());
    // println!("day 2 b: {}", day2::decode_file_part_2());
    // println!("day 3 a: {}", day3::check_parts(&read_aoc_lines("3")));
    // println!("day 3 b: {}", day3::calc_gear_ratio(&read_aoc_lines("3")));
    // println!("day 4 a: {}", day4::sum_points(&read_aoc_lines("4")));
    // println!("day 4 b: {}", day4::sum_card_copies(&read_aoc_lines("4")));
    // println!("day 5 a: {}", day5::nearest_loacation(&read_aoc_lines("5"), day5::line_to_seeds));
    // println!("day 5 b: {}", day5::nearest_loacation_b(&read_aoc_lines("5")));
    println!("day 6 a: {}", day6::solve(&read_aoc_lines("6")));
    println!("day 6 b: {}", day6::solve(&read_aoc_lines("6b")));

}

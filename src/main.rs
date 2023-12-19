pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod util;
use aoc_23::util::io::read_aoc_lines;

fn main() {

    // println!("{}", day1::decode_file_b());
    // println!("day 2 a: {}", day2::decode_file());
    // println!("day 2 b: {}", day2::decode_file_part_2());
    // println!("day 3 a: {}", day3::check_parts(&read_aoc_lines("3")));
    // println!("day 3 b: {}", day3::calc_gear_ratio(&read_aoc_lines("3")));
    println!("day 4 a: {}", day4::sum_points(&read_aoc_lines("4")));
    println!("day 4 b: {}", day4::sum_card_copies(&read_aoc_lines("4")));
}

pub mod day1;
pub mod day2;
pub mod util;
use aoc_23::util::io::read_aoc_lines;

fn main() {

    // println!("{}", day1::decode_file_b());
    println!("day 2 a: {}", day2::decode_file());
    println!("day 2 b: {}", day2::decode_file_part_2());
}

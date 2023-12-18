extern crate advent_of_code_2023_rust;

#[cfg(test)]
mod day1_tests{
    use advent_of_code_2023_rust::day1::foo;

    #[test]
    fn it_adds_two() {
        assert_eq!("foo", foo());
    }
}


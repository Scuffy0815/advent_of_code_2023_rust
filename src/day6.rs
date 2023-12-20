#[derive(PartialEq, Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn button_times_to_distance(&self) -> Vec<(i64, i64)> {
        return (0..self.time + 1)
            .map(|button_time| (button_time, distance_covered(&button_time, &self.time)))
            .collect();
    }

    fn beatable_races(&self) -> i64 {
        self.button_times_to_distance()
            .iter()
            .filter(|(_, distance)| *distance > self.distance)
            .count() as i64
    }
}

fn numbers_from_line(line: &str) -> Vec<i64> {
    return line.split(" ")
        .filter(|s| !s.is_empty())
        .filter(|s| !s.chars().any(|c| c.is_alphabetic()))
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
}

fn parse_races(input: &Vec<String>) -> Vec<Race> {
    let mut lines = input.iter();
    let times = numbers_from_line(lines.next().unwrap());
    let distances = numbers_from_line(lines.next().unwrap());

    let mut races = vec![];
    for (time, distance) in times.iter().zip(distances.iter()) {
        println!("{} {}", time, distance);
        races.push(Race { time: *time, distance: *distance });
    }
    return races;
}

fn distance_covered(button_time: &i64, race_time: &i64) -> i64 {
    return button_time * (race_time - button_time);
}

pub fn solve(input: &Vec<String>) -> i64 {
    let races = parse_races(input);
    return races.iter()
        .map(|r| r.beatable_races())
        .fold(1, |acc, x| acc * x);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_races() {
        let input: Vec<String> =
            "Time:      7  15   30
Distance:  9  40  200".to_string().split("\n").map(|s| s.to_string()).collect();

        let expected_races = vec![
            Race { time: 7, distance: 9 },
            Race { time: 15, distance: 40 },
            Race { time: 30, distance: 200 },
        ];

        assert_eq!(expected_races, parse_races(&input))
    }


    #[test]
    fn should_calc_times() {
        let race = Race { time: 7, distance: 9 };

        let expeted_times_to_distance = vec![(0, 0), (1, 6), (2, 10), (3, 12), (4, 12), (5, 10), (6, 6), (7, 0)];
        assert_eq!(expeted_times_to_distance, race.button_times_to_distance())
    }

    #[test]
    fn should_calc_beatable() {
        assert_eq!(4, Race { time: 7, distance: 9 }.beatable_races());
        assert_eq!(8, Race { time: 15, distance: 40 }.beatable_races());
        assert_eq!(9, Race { time: 30, distance: 200 }.beatable_races())
    }


    #[test]
    fn should_solve_a() {
        let input: Vec<String> =
            "Time:      7  15   30
Distance:  9  40  200".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(288, solve(&input));
    }
    #[test]
    fn should_solve_b() {
        let input: Vec<String> =
            "Time:      71530
Distance:  940200".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(71503, solve(&input));
    }
}

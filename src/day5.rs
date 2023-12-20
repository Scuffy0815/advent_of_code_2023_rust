#[derive(Debug, PartialEq, Clone)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn new(mappings: Vec<Mapping>) -> Map {
        return Map { mappings };
    }

    fn get(&self, source: &i64) -> i64 {
        let mut result = *source;
        for mapping in &self.mappings {
            result = mapping.map(*source);
            if result != *source {
                break;
            }
        }
        return result;
    }
    fn rev_get(&self, destination: &i64) -> i64 {
        let mut result = *destination;
        for mapping in &self.mappings {
            result = mapping.rev_map(*destination);
            if result != *destination {
                break;
            }
        }
        return result;
    }
}


#[derive(Debug, PartialEq, Clone)]
struct Mapping {
    min_source: i64,
    max_source: i64,
    min_destination: i64,
    max_destination: i64,
    diff: i64,
}

impl Mapping {
    fn new(destination_start: i64, source_start: i64, range: i64) -> Mapping {
        let diff = destination_start - source_start;
        let min_source = source_start;
        let max_source = source_start + range;
        return Mapping { min_source, max_source, min_destination: destination_start, max_destination: destination_start + range, diff };
    }

    fn map(&self, source: i64) -> i64 {
        if source < self.min_source || source > self.max_source {
            return source;
        }
        return source + self.diff;
    }

    fn rev_map(&self, destination: i64) -> i64 {
        if destination < self.min_destination || destination > self.max_destination {
            return destination;
        }
        return destination - self.diff;
    }
}

struct SeedRange {
    start: i64,
    end: i64,
}

impl SeedRange {
    fn new(start: i64, end: i64) -> SeedRange {
        return SeedRange { start, end };
    }

    fn contains(&self, seed: i64) -> bool {
        return seed >= self.start && seed <= self.end;
    }
}

fn seeds_contained(seed_ranges: &Vec<SeedRange>, &value: &i64) -> bool {
    return seed_ranges.iter().any(|range| range.contains(value));
}


pub fn line_to_seeds(line: &String) -> Vec<i64> {
    return line.split(" ")
        .filter(|s| !s.is_empty())
        .filter(|s| !s.chars().any(|c| c.is_alphabetic()))
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
}

fn line_to_seeds_b(line: &String) -> Vec<SeedRange> {
    let mut numbers = line.split(" ")
        .filter(|s| !s.is_empty())
        .filter(|s| !s.chars().any(|c| c.is_alphabetic()))
        .map(|s| s.parse::<i64>().unwrap());

    let mut seeds = vec![];

    while let Some(seed_start) = numbers.next() {
        let range = numbers.next();
        seeds.push(SeedRange::new(seed_start, seed_start + range.unwrap()));
    }
    return seeds;
}

fn line_to_mapping(line: &String) -> Mapping {
    let mut fuck_this = line.split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .take(3);
    // this is shitty again. I can't deconstruct so yay have to do it manually
    let destination_start = fuck_this.next().unwrap();
    let source_start = fuck_this.next().unwrap();
    let range = fuck_this.next().unwrap();

    return Mapping::new(destination_start, source_start, range);
}

fn apply_mappings(seed: i64, maps: &Vec<Map>) -> i64 {
    let mut result = seed;
    for mapping in maps {
        result = mapping.get(&result)
    }
    return result;
}
fn rev_apply_mappings(seed: i64, maps: &Vec<Map>) -> i64 {
    let mut result = seed;
    let mut path = vec![seed];
    for mapping in maps {
        result = mapping.rev_get(&result);
        path.push(result)
    }
    return result;
}

pub fn nearest_loacation(input: &Vec<String>, seed_func: fn(&String) -> Vec<i64>) -> i64 {
    let mut lines = input.iter();
    let seeds_line = lines.next();
    let seeds = seed_func(seeds_line.unwrap());
    let mut current_mappings = vec![];
    let mut maps: Vec<Map> = vec![];

    while let Some(line) = lines.next() {
        let start_char = line.chars().next().unwrap_or(' ');
        if start_char.is_alphabetic() && !current_mappings.is_empty() {
            maps.push(Map::new(current_mappings));
            current_mappings = vec![];
        } else if start_char.is_digit(10) {
            let mapping_from_line = line_to_mapping(line);
            current_mappings.push(mapping_from_line);
        }
    }
    maps.push(Map::new(current_mappings));




    println!("seeds: {}", seeds.len());
    let foo: Vec<i64> = seeds.iter().map(|seed| apply_mappings(*seed, &maps)).collect();

    return *foo.iter().min().unwrap();
}
pub fn nearest_loacation_b(input: &Vec<String>) -> i64 {
    let mut lines = input.iter();
    let seeds_line = lines.next();
    let seeds = line_to_seeds_b(seeds_line.unwrap());
    let mut current_mappings = vec![];
    let mut maps: Vec<Map> = vec![];

    while let Some(line) = lines.next() {
        let start_char = line.chars().next().unwrap_or(' ');
        if start_char.is_alphabetic() && !current_mappings.is_empty() {
            maps.push(Map::new(current_mappings));
            current_mappings = vec![];
        } else if start_char.is_digit(10) {
            let mapping_from_line = line_to_mapping(line);
            current_mappings.push(mapping_from_line);
        }
    }
    maps.push(Map::new(current_mappings));
    let mut maps_reverse = maps.clone();
    maps_reverse.reverse();

    let mut result: i64 = -1;
    let mut i: i64 = 0;
    while result == -1 {
        if i % 1000000 == 0 {
            println!("i: {}", i);
        }
        let candidate = rev_apply_mappings(i, &maps_reverse);
        if seeds_contained(&seeds, &candidate) {
            result = i;
        }
        i += 1;
    }


    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extracts_seed() {
        let line = "seeds: 79 14 55 13".to_string();

        let expected = vec![79, 14, 55, 13];
        assert_eq!(expected, line_to_seeds(&line));
    }


    #[test]
    fn should_convert_range_line_to_map() {
        let line = "50 98 2".to_string();

        let expected: Mapping = Mapping::new(50, 98, 2);

        assert_eq!(expected, line_to_mapping(&line));
    }

    #[test]
    fn should_apply_mappings() {
        let map = Map::new(vec![
            Mapping::new(50, 98, 2),
            Mapping::new(52, 50, 48), ]);


        assert_eq!(81, map.get(&79));
        assert_eq!(14, map.get(&14));
        assert_eq!(57, map.get(&55));
        assert_eq!(13, map.get(&13));
    }

    #[test]
    fn should_calculate_min_positions() {
        let input: Vec<String> =
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string().split("\n").map(|s| s.to_string()).collect();

        assert_eq!(35, nearest_loacation(&input, line_to_seeds));
        assert_eq!(46, nearest_loacation_b(&input));
    }
}

use std::collections::{HashMap};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AlmanacMapping {
    destination_start: u32,
    source_start: u32,
    range: u32
}

impl FromStr for AlmanacMapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_whitespace().collect::<Vec<&str>>();

        Ok(AlmanacMapping {
            destination_start: parts[0].parse::<u32>().unwrap(),
            source_start: parts[1].parse::<u32>().unwrap(),
            range: parts[2].parse::<u32>().unwrap()
        })
    }
}

impl AlmanacMapping {
    pub fn value_for(&self, source: u32) -> Result<u32, ()> {
        if source < self.source_start || source >= self.source_start + self.range {
            return Err(());
        }

        Ok(self.destination_start + (source - self.source_start))
    }
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<u32>,
    maps: HashMap<String, Vec<AlmanacMapping>>
}

impl Almanac {
    pub fn seed_location(&self, seed: u32) -> u32 {
        let order = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
        let mut current_value = seed;

        for i in 1..order.len() {
            let map_key = format!("{}-to-{}", order[i-1], order[i]).to_string();

            let new_value = self.maps[&map_key]
                .iter()
                .map(|m| m.value_for(current_value))
                .find(|v| v.is_ok())
                .unwrap_or(Ok(current_value)).unwrap();

            current_value = new_value;
        }

        current_value
    }
}

#[aoc_generator(day5)]
pub fn input_almanac(input: &str) -> Almanac {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();

    let seeds = seeds_str.split_once(": ").unwrap().1.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let maps =
        maps_str
            .split("\n\n")
            .map(|l| {
                let (map_type, map_data) = l.split_once("\n").unwrap();
                (map_type.split_once(' ').unwrap().0.to_string(), map_data.lines().map(|l| AlmanacMapping::from_str(l).unwrap()).collect::<Vec<AlmanacMapping>>())
            })
            .collect::<HashMap<String, Vec<AlmanacMapping>>>();

    Almanac {
        seeds,
        maps
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Almanac) -> u32 {
    input.seeds.iter().map(|s| input.seed_location(*s)).min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Almanac) -> u32 {
    let mut min_val = u32::MAX;

    for i in (0..input.seeds.len()).step_by(2) {
        let (seed_range_start, seed_range) = (input.seeds[i], input.seeds[i+1]);

        (0..seed_range).map(|s| input.seed_location(seed_range_start + s)).for_each(|s| {
            if s < min_val {
                min_val = s;
            }
        });
    }

    min_val
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&super::input_almanac(TEST_INPUT)), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&super::input_almanac(TEST_INPUT)), 46);
    }
}
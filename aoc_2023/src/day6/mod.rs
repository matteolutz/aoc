pub fn find_race_possibilities(race_time: &u64, race_record_distance: &u64) -> u64 {
    let mut possibilities = 0;

    for i in 0..*race_time {
        let driving_time = race_time - i;
        let distance = i * driving_time;

        if distance > *race_record_distance {
            possibilities += 1;
        }
    }

    possibilities
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let (times, record_distances) = input.split_once('\n').unwrap();

    let times = times.split_once(": ").unwrap().1.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let record_distances = record_distances.split_once(": ").unwrap().1.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let races = times.into_iter().zip(record_distances.into_iter()).collect::<Vec<_>>();

    races
        .iter()
        .map(|(race_time, race_record_distance)| find_race_possibilities(&(*race_time as u64), &(*race_record_distance as u64)))
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let (times, record_distances) = input.split_once('\n').unwrap();
    let race_time = times.split_once(": ").unwrap().1.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap();
    let race_record_distance = record_distances.split_once(": ").unwrap().1.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap();

    find_race_possibilities(&race_time, &race_record_distance)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1("Time:      7  15   30
Distance:  9  40  200
"), 288);
    }


    #[test]
    fn test_part2() {
        assert_eq!(super::part2("Time:      7  15   30
Distance:  9  40  200
"), 71503);
    }
}
use std::ops::Range;

#[aoc_generator(day4)]
pub fn input_pairs(input: &str) -> Vec<(Range<u32>, Range<u32>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (elve1, elve2) = line.split_once(',').unwrap();

            let (elve1start, elve1end) = elve1.split_once('-').unwrap();
            let (elve2start, elve2end) = elve2.split_once('-').unwrap();

            (
                elve1start.parse::<u32>().unwrap()..elve1end.parse::<u32>().unwrap(),
                elve2start.parse::<u32>().unwrap()..elve2end.parse::<u32>().unwrap(),
            )
        })
        .collect()
}

pub fn is_full_overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start <= b.start && a.end >= b.end
        || b.start <= a.start && b.end >= a.end
}

#[aoc(day4, part1)]
pub fn part1(pairs: &[(Range<u32>, Range<u32>)]) -> u32 {
    pairs
        .iter().fold(0, |acc, (a, b)| {
            if is_full_overlap(a, b) {
                acc + 1
            } else {
                acc
            }
        })
}

pub fn is_partial_overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start.max(b.start) <= a.end.min(b.end)
}

#[aoc(day4, part2)]
pub fn part2(pairs: &[(Range<u32>, Range<u32>)]) -> u32 {
    pairs
        .iter().fold(0, |acc, (a, b)| {
            if is_partial_overlap(a, b) {
                acc + 1
            } else {
                acc
            }
        })
}
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_data_stream(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<char>>()
}

#[aoc(day6, part1)]
pub fn part1(input: &[char]) -> usize {
    let mut latest = Vec::<char>::with_capacity(4);
    for (index, c) in input.iter().enumerate() {
        if index < 4 {
            latest.push(*c);
            continue;
        }

        let mut uniqu = HashSet::new();
        if latest.iter().all(|c| uniqu.insert(c)) {
            return index;
        }

        latest.remove(0);
        latest.push(*c);
    }

    0
}

#[aoc(day6, part2)]
pub fn part2(input: &[char]) -> usize {
    let mut latest = Vec::<char>::with_capacity(14);
    for (index, c) in input.iter().enumerate() {
        if index < 14 {
            latest.push(*c);
            continue;
        }

        let mut uniqu = HashSet::new();
        if latest.iter().all(|c| uniqu.insert(c)) {
            return index;
        }

        latest.remove(0);
        latest.push(*c);
    }

    0
}

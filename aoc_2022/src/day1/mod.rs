#[aoc_generator(day1)]
pub fn input_calories(input: &str) -> Vec<u32> {
    let mut t = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split('\n')
            .fold(0u32, |acc, line| acc + line.parse::<u32>().unwrap())
        )
        .collect::<Vec<u32>>();
    t.sort_by(|a, b| b.cmp(a));
    t
}

#[aoc(day1, part1)]
pub fn part1(calories: &[u32]) -> u32 {
    *calories.first().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(calories: &[u32]) -> u32 {
    calories.iter().take(3).sum()
}

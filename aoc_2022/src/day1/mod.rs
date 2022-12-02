#[aoc_generator(day1)]
pub fn input_calories(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split('\n').map(|s| s.parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>()
}

#[aoc(day1, part1)]
pub fn part1(calories: &[Vec<i32>]) -> i32 {
    calories.iter().map(|elve| elve.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(calories: &[Vec<i32>]) -> i32 {
    let mut elves_total = calories
        .iter()
        .map(|elve| elve.iter().sum())
        .collect::<Vec<i32>>();

    elves_total.sort();
    elves_total.reverse();

    elves_total[0] + elves_total[1] + elves_total[2]
}

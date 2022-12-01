#[aoc_generator(day1)]
pub fn input_calories(input: &str) -> Vec<Vec<i32>> {
    let mut calories = Vec::new();

    let mut current_elve = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            calories.push(current_elve);
            current_elve = Vec::new();
        } else {
            current_elve.push(l.parse().unwrap());
        }
    }

    calories
}

#[aoc(day1, part1)]
pub fn part1(calories: &[Vec<i32>]) -> i32 {
    let mut max = 0;
    for elve in calories {
        let mut total = 0;
        for c in elve {
            total += c;
        }
        if total > max {
            max = total;
        }
    }
    max
}

#[aoc(day1, part2)]
pub fn part2(calories: &[Vec<i32>]) -> i32 {
    let mut elves_total = Vec::new();
    for elve in calories {
        let mut total = 0;
        for c in elve {
            total += c;
        }
        elves_total.push(total);
    }

    elves_total.sort();
    elves_total.reverse();

    elves_total[0] + elves_total[1] + elves_total[2]
}

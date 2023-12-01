pub fn get_digits(input: &str) -> u32 {
    let mut first: Option<u32> = Option::None;
    let mut second: Option<u32> = Option::None;

    for c in input.chars() {
        if !c.is_digit(10) {
            continue;
        }

        first = Some(c.to_digit(10).unwrap());
        break;
    }

    for c in input.chars().rev() {
        if !c.is_digit(10) {
            continue;
        }

        second = Some(c.to_digit(10).unwrap());
        break;
    }

    (first.unwrap_or(0) * 10) + second.unwrap_or(first.unwrap_or(0))
}

#[aoc_generator(day1)]
pub fn input_calibration_data(input: &str) -> Vec<u32> {
    input
        .lines()
        .into_iter()
        .map(|line| get_digits(line))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(calibration_data: &[u32]) -> u32 {
    println!("Calibration data: {:?}", calibration_data);
    calibration_data
        .iter()
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(calibration_data: &[u32]) -> u32 {
    0
}

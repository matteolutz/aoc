use regex::Regex;


const DIGIT_MAP: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn get_digits(input: &str) -> u32 {
    let find_last_digit_regex: Regex = Regex::new(r"(.*)(?P<digit>one|two|three|four|five|six|seven|eight|nine)(.*)").unwrap();
    let find_digit_regex: Regex = Regex::new(r"(?P<digit>one|two|three|four|five|six|seven|eight|nine)(.*)").unwrap();

    let mut first: Option<(u32, usize)> = None;
    let mut second: Option<(u32, usize)> = None;

    for (i, c) in input.chars().enumerate() {
        if !c.is_digit(10) {
            continue;
        }

        let d = c.to_digit(10).unwrap();

        if first.is_none() {
            first = Some((d, i));
            continue;
        }

        second = Some((d, i));
    }

    if let Some(m) = find_last_digit_regex.captures(input) {
        if let Some(digit) = m.name("digit") {
            let idx = digit.start();
            if (second.is_none() && first.is_some() && idx > first.unwrap().1) || (second.is_some() && idx > second.unwrap().1) {
                second =
                    Some((DIGIT_MAP.iter().find(|(k, _)| k == &digit.as_str()).unwrap().1, idx));
            }
        }
    }

    if let Some(m) = find_digit_regex.captures(input) {
        if let Some(digit) = m.name("digit") {
            let idx = digit.start();
            if first.is_none() || idx < first.unwrap().1 {
                first =
                    Some((DIGIT_MAP.iter().find(|(k, _)| k == &digit.as_str()).unwrap().1, idx));
            }
        }
    }

    (first.unwrap_or((0, 0)).0 * 10) + second.unwrap_or(first.unwrap_or((0, 0))).0
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
    calibration_data
        .iter()
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(calibration_data: &[u32]) -> u32 {
    calibration_data
        .iter()
        .sum()
}

// add tests here
#[cfg(test)]
mod tests {
    #[test]
    fn test_get_digits() {
        /*assert_eq!(super::get_digits("two1nine"), 29);
        assert_eq!(super::get_digits("eightwothree"), 83);
        assert_eq!(super::get_digits("abcone2threexyz"), 13);
        assert_eq!(super::get_digits("xtwone3four"), 24);
        assert_eq!(super::get_digits("4nineeightseven2"), 42);
        assert_eq!(super::get_digits("zoneight234"), 14);
        assert_eq!(super::get_digits("7pqrstsixteen"), 76);*/
        assert_eq!(super::get_digits("nineight1"), 98);
    }
}
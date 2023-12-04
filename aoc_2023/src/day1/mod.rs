use regex::{Captures, Regex};


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

pub fn get_digits(input: &str, allow_strings: bool) -> u32 {
    let find_last_digit_regex: Regex = Regex::new(r"(.*)(?P<digit>one|two|three|four|five|six|seven|eight|nine)(.*)").unwrap();
    let find_digit_regex: Regex = Regex::new(r"(?P<digit>one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut processed_input = if allow_strings { find_last_digit_regex.replace_all(input, |captures: &Captures| {
        let digit = captures.name("digit").unwrap().as_str();
        let digit_value = DIGIT_MAP.iter().find(|(d, _)| d == &digit).unwrap().1;
        format!("{}{}{}x{}", captures.get(1).unwrap().as_str(), digit, digit_value, captures.get(3).unwrap().as_str())
    }).to_string() } else { input.to_string() };

    if allow_strings {
        processed_input = find_digit_regex.replace(&processed_input, |captures: &Captures| {
            let digit = captures.name("digit").unwrap().as_str();
            let digit_value = DIGIT_MAP.iter().find(|(d, _)| d == &digit).unwrap().1;
            format!("{}{}x", digit, digit_value)
        }).to_string();
    }

    let mut first: Option<(u32, usize)> = None;
    let mut second: Option<(u32, usize)> = None;

    for (i, c) in processed_input.chars().enumerate() {
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

    (first.unwrap_or((0, 0)).0 * 10) + second.unwrap_or(first.unwrap_or((0, 0))).0
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|line| get_digits(line, false))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|line| get_digits(line, true))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_digits() {
        assert_eq!(super::get_digits("two1nine", true), 29);
        assert_eq!(super::get_digits("eightwothree", true), 83);
        assert_eq!(super::get_digits("abcone2threexyz", true), 13);
        assert_eq!(super::get_digits("xtwone3four", true), 24);
        assert_eq!(super::get_digits("4nineeightseven2", true), 42);
        assert_eq!(super::get_digits("zoneight234", true), 14);
        assert_eq!(super::get_digits("7pqrstsixteen", true), 76);
        assert_eq!(super::get_digits("nineight", true), 98);
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), 281);
    }

}
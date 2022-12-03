#[aoc_generator(day3)]
pub fn input_rucksacks(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|l| {
            let chars = l.chars().collect::<Vec<char>>();
            assert_eq!(chars.len() % 2, 0);
            chars
        }).collect()
}

pub fn find_common_item(a: &[char], b: &[char]) -> Option<char> {
    for c in a {
        if b.contains(c) {
            return Some(*c);
        }
    }
    None
}

pub fn get_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        (*c as u32) - ('A' as u32) + 27
    } else {
        (*c as u32) - ('a' as u32) + 1
    }
}

#[aoc(day3, part1)]
pub fn part1(rucksacks: &[Vec<char>]) -> u32 {
    rucksacks
        .iter().fold(0, |acc, rucksack| {
            if let Some(i) = find_common_item(&rucksack[0..rucksack.len()/2], &rucksack[rucksack.len()/2..]) {
                acc + get_priority(&i)
            } else {
                acc
            }
        })
}

pub fn find_badge(a: &[char], b: &[char], c: &[char]) -> Option<char> {
    for badge in a {
        if b.contains(badge) && c.contains(badge) {
            return Some(*badge);
        }
    }
    None
}

#[aoc(day3, part2)]
pub fn part2(rucksacks: &[Vec<char>]) -> u32 {
    assert_eq!(rucksacks.len() % 3, 0);

    let mut count = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        if let Some(b) = find_badge(
            &rucksacks[i],
            &rucksacks[i+1],
            &rucksacks[i+2]
        ) {
            count += get_priority(&b);
        }
    }

    count
}
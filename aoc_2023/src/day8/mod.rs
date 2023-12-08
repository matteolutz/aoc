use std::collections::HashMap;

#[aoc_generator(day8)]
pub fn input_network(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let network = network.lines().map(|line| {
        let (origin, destinations) = line.split_once(" = ").unwrap();
        let (left, right) = destinations.split_once(", ").unwrap();

        (origin.to_string(), (left.trim_start_matches('(').to_string(), right.trim_end_matches(')').to_string()))
    }).collect::<HashMap<String, (String, String)>>();

    (instructions.chars().collect(), network)
}

#[aoc(day8, part1)]
pub fn part1((instructions, network): &(Vec<char>, HashMap<String, (String, String)>)) -> u32 {
    let mut acc = 0;
    let mut current = "AAA";

    while current != "ZZZ" {
        let inst = instructions[(acc as usize) % instructions.len()];
        if inst == 'L' {
            current = &network[current].0;
        } else if inst == 'R' {
            current = &network[current].1;
        } else {
            panic!("Invalid instruction: {}", inst);
        }
        acc += 1;
    }

    acc
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[aoc(day8, part2)]
pub fn part2((instructions, network): &(Vec<char>, HashMap<String, (String, String)>)) -> u64 {
    let words = network.iter().map(|(k, _)| k.to_string()).filter(|w| w.ends_with('A')).collect::<Vec<String>>();
    let accs = words.iter().map(|w| {
        let mut acc = 0;
        let mut current = w;

        while !current.ends_with('Z') {
            let inst = instructions[(acc as usize) % instructions.len()];
            if inst == 'L' {
                current = &network[current].0;
            } else if inst == 'R' {
                current = &network[current].1;
            } else {
                panic!("Invalid instruction: {}", inst);
            }
            acc += 1;
        }

        acc
    }).collect::<Vec<u64>>();

    let mut acc = accs[0];
    for i in 1..accs.len() {
        acc = lcm(acc, accs[i]);
    }

    acc
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let network = super::input_network("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(super::part1(&network), 6);
    }


    #[test]
    fn test_part2() {
        let network = super::input_network("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(super::part2(&network), 6);
    }
}
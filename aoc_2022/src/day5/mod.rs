#[derive(Debug)]
pub struct MoveInstruction {
    amount: u32,
    from: u32,
    to: u32
}

#[aoc_generator(day5)]
pub fn input_stacks(input: &str) -> (Vec<Vec<char>>, Vec<MoveInstruction>) {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stack_lines = stacks.lines().collect::<Vec<&str>>();
    let last_line = stack_lines.pop().unwrap();

    let num_stacks = last_line.split(' ').filter(|p| !p.is_empty()).count();

    let mut stacks = Vec::<Vec<char>>::with_capacity(num_stacks);

    for (index, ch) in last_line.chars().enumerate() {
        if ch.is_numeric() {
            let mut stack = Vec::<char>::new();
            for line in stack_lines.iter().rev() {
                let ch = line.chars().nth(index);
                if ch.is_none() || ch.unwrap() == ' ' {
                    break;
                }
                stack.push(ch.unwrap());
            }
            stacks.push(stack);
        }
    }

    let is =
        instructions
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<&str>>();
                MoveInstruction {
                    amount: parts[1].parse::<u32>().unwrap(),
                    from: parts[3].parse::<u32>().unwrap(),
                    to: parts[5].parse::<u32>().unwrap()
                }
            })
            .collect::<Vec<MoveInstruction>>();

    (stacks, is)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<char>>, Vec<MoveInstruction>)) -> String {
    let (stacks, instructions) = input;

    let mut stacks = stacks.clone();

    for instruction in instructions {
        for _ in 0..instruction.amount {
            let ch = stacks[instruction.from as usize - 1].pop().unwrap();
            stacks[instruction.to as usize - 1].push(ch);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Vec<char>>, Vec<MoveInstruction>)) -> String {
    let (stacks, instructions) = input;

    let mut stacks = stacks.clone();

    for instruction in instructions {

        let mut temp = Vec::<char>::with_capacity(instruction.amount as usize);
        for _ in 0..instruction.amount {
            let ch = stacks[instruction.from as usize - 1].pop().unwrap();
            temp.push(ch);
        }
        for ch in temp.iter().rev() {
            stacks[instruction.to as usize - 1].push(*ch);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

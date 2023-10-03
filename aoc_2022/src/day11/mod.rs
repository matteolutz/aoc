const PART_1_TOTAL_ROUNDS: u32 = 20;

#[derive(Debug, Clone)]
pub enum Operation {
    MultiplySelf,
    Multiply(u32),
    AddSelf,
    Add(u32)
}

#[derive(Debug, Clone)]
pub struct Test {
    divisible_by: u32,
    throw_to_true: u32,
    throw_to_false: u32
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: Test
}

#[aoc_generator(day11)]
pub fn input_monkey_data(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let lines = l.lines().collect::<Vec<&str>>();
            let items = lines[1]
                .split_once(':')
                .unwrap().1
                .split(", ")
                .map(|i| i.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let operation_parts = lines[2].split_once('=').unwrap().1.trim()
                .split_whitespace().collect::<Vec<&str>>();
            let operation_fn = match operation_parts[1] {
                "+" => {
                    if operation_parts[2] == "old" {
                        Operation::AddSelf
                    } else {
                        let val = operation_parts[2].parse::<u32>().unwrap();
                        Operation::Add(val)
                    }
                },
                "*" => {
                    if operation_parts[2] == "old" {
                        Operation::MultiplySelf
                    } else {
                        let val = operation_parts[2].parse::<u32>().unwrap();
                        Operation::Multiply(val)
                    }
                },
                _ => panic!("Unknown operation"),
            };

            let test_fn = Test {
                divisible_by: lines[3].split_once("by").unwrap().1.trim().parse::<u32>().unwrap(),
                throw_to_true: lines[4].split_once("monkey").unwrap().1.trim().parse::<u32>().unwrap(),
                throw_to_false: lines[5].split_once("monkey").unwrap().1.trim().parse::<u32>().unwrap(),
            };

            Monkey {
                items,
                operation: operation_fn,
                test: test_fn
            }

        })
        .collect::<Vec<Monkey>>()
}

pub fn eval_operation(operation: &Operation, val: u32) -> u32 {
    match operation {
        Operation::MultiplySelf => val * val,
        Operation::Multiply(val2) => val * val2,
        Operation::AddSelf => val + val,
        Operation::Add(val2) => val + val2,
    }
}

pub fn inspect_item(item: &u32, operation: Operation) -> u32 {
    eval_operation(&operation, *item) / 3
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> u32 {
    let mut monkeys = input.to_vec();
    let mut monkey_items =
        monkeys.iter().map(|m| m.items.clone()).collect::<Vec<Vec<u32>>>();

    for i in 0..PART_1_TOTAL_ROUNDS {
        for (monkey_index, monkey) in monkeys.iter().enumerate() {
            for item_index in 0..monkey_items[monkey_index].len() {
                let item = monkey_items[monkey_index][item_index];
                let new_item = inspect_item(&item, monkey.operation.clone());
                monkey_items[
                    if new_item % monkey.test.divisible_by == 0 {
                        monkey.test.throw_to_true
                    } else {
                        monkey.test.throw_to_false
                    } as usize
                ].push(new_item);
            }
        }
        println!("Round {} complete", i+1);
    }

    0
}
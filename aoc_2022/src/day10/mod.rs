const INTERESTING_CYCLES: [u32; 6] = [
    20, 60, 100, 140, 180, 220
];

const CRT_WIDTH: u32 = 40;
const CRT_HEIGHT: u32 = 6;

#[derive(Debug)]
pub enum Instruction {
    AddX(i32),
    Noop
}

#[aoc_generator(day10)]
pub fn input_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let (instruction, attribute) = line.split_once(' ').unwrap_or(("noop", ""));
        match instruction {
            "addx" => Instruction::AddX(attribute.parse::<i32>().unwrap()),
            _ => Instruction::Noop
        }
    }).collect()
}

pub fn run_cycle(clock: &u32, x_reg: &i32) -> i32 {
    if INTERESTING_CYCLES.contains(&clock) {
        return *x_reg * *clock as i32;
    }
    0
}

#[aoc(day10, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut clock: u32 = 0;
    let mut x_reg: i32 = 1;
    let mut sum: i32 = 0;

    for instruction in instructions {
        match instruction {
            Instruction::AddX(value) => {
                for _ in 0..2 {
                    clock += 1;
                    sum += run_cycle(&clock, &x_reg);
                }
                x_reg += value;
            },
            Instruction::Noop => {
                clock += 1;
                sum += run_cycle(&clock, &x_reg);
            }
        }
    }

    sum
}

pub fn draw_cycle(clock: &u32, sprite_pos: &i32, screen: &mut Vec<char>) -> () {
    let is_pixel_visible = (((*clock as i32 - 1) % CRT_WIDTH as i32) - *sprite_pos).abs() < 2;
    screen.push(if is_pixel_visible { '#' } else { '.' });
}

#[aoc(day10, part2)]
pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut clock: u32 = 0;
    let mut x_reg: i32 = 1;
    let mut screen: Vec<char> = Vec::<char>::with_capacity((CRT_WIDTH * CRT_HEIGHT) as usize);

    for instruction in instructions {
        match instruction {
            Instruction::AddX(value) => {
                for _ in 0..2 {
                    clock += 1;
                    draw_cycle(&clock, &x_reg, &mut screen);
                }
                x_reg += value;
            },
            Instruction::Noop => {
                clock += 1;
                draw_cycle(&clock, &x_reg, &mut screen);
            }
        }
    }

    for (i, pixel) in screen.iter().enumerate() {
        if i % CRT_WIDTH as usize == 0 {
            println!();
        }
        print!("{}", pixel);
    }
    println!();

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_part2() {
        part2(
            &input_instructions("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop")
        );
    }
}
#[aoc_generator(day12)]
pub fn input_heightmap_data(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.as_bytes().to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
}

#[aoc(day12, part1)]
pub fn part1(heightmap: &Vec<Vec<u8>>) -> u32 {
    println!("{:?}", heightmap);


    0
}
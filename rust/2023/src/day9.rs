

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i16>> {
    input.lines()
        .map(|l| l.split_whitespace()
            .map(|n| n.parse::<i16>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
pub fn solve_part1(sequences: &Vec<Vec<i16>>) -> u32 {
    for seq in sequences.iter() {
    }
    return 0
}


#[aoc(day9, part2)]
pub fn solve_part2(_sequences: &Vec<Vec<i16>>) -> u64 {

    return 0
}
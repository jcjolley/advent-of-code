

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|l| l.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
pub fn solve_part1(sequences: &Vec<Vec<i32>>) -> i32 {
    sequences.iter().map(|seq| {
        predict_next(seq)
    }).sum()
}

pub fn predict_next(seq: &Vec<i32>) -> i32 {
    let mut derivations = Vec::new();
    let mut derivation = seq.clone();
    while !derivation.iter().all(|v| *v == 0) {
        derivations.push(derivation.clone());
        derivation = derivation.windows(2).map(|x| x[1].clone() - x[0].clone()).collect::<Vec<_>>();
    }
    derivations.iter().map(|d| d.last().unwrap()).sum::<i32>()
}

fn print_vec(v: &Vec<i32>, label: &str) {
    print!("{label}: ");
    for n in v.iter() {
        print!("{n} ")
    }
}


#[aoc(day9, part2)]
pub fn solve_part2(sequences: &Vec<Vec<i32>>) -> i32 {
    sequences.iter().map(|seq| predict_previous(seq) ).sum()
}

pub fn predict_previous(seq: &Vec<i32>) -> i32 {
    let mut derivations = Vec::new();
    let mut derivation = seq.clone();
    derivation.reverse();
    while !derivation.iter().all(|v| *v == 0) {
        derivations.push(derivation.clone());
        derivation = derivation.windows(2).map(|x| x[1].clone() - x[0].clone()).collect::<Vec<_>>();
    }
    derivations.iter().map(|d| d.last().unwrap()).sum::<i32>()

}
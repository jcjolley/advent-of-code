
pub struct Race {
    time: u64,
    record: u64
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut races = Vec::new();
    let vecs = input.lines()
        .map(|l| l.split_whitespace().skip(1).map(|w| w.parse::<u64>().unwrap()).collect::<Vec<_>>() )
        .collect::<Vec<_>>();

    for i in 0..vecs[0].len() {
        races.push(Race {
            time: vecs[0][i],
            record: vecs[1][i]
        })
    }

    races
        .iter()
        .map(|race|
            (0..race.time)
                .map(|t| calculate_distance(t, race.time))
                .filter(|d| d > &race.record)
                .collect::<Vec<_>>()
                .len()
        )
        .reduce(|a, x| a * x)
        .unwrap()
}

fn calculate_distance(held: u64, time: u64) -> u64 {
    assert!(held <= time);
    let travel_time = time - held;
    held * travel_time
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let vecs = input.lines()
        .map(|l| l
            .split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
        )
        .collect::<Vec<_>>();

    let race = Race {
        time: vecs[0],
        record: vecs[1]
    };

    (0..race.time)
        .map(|t| calculate_distance(t, race.time))
        .filter(|d| d > &race.record)
        .collect::<Vec<_>>()
        .len()
}

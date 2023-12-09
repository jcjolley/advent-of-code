pub struct Card {
    id: u32,
    winners: Vec<u8>,
    mine: Vec<u8>,
}

const NUM_CARDS: usize = 196;

impl Card {
    fn from(line: &str) -> Self {
        let mut parts = line.split(':');
        let id = parts.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let mut value_parts = parts.next().unwrap().split('|');
        let winners= value_parts.next().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect();
        let mine = value_parts.next().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect();

        Self {
            id,
            winners,
            mine
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card>{
    input.lines().map(|line| Card::from(line)).collect()
}

fn calc_points(card: &Card) -> u32 {
    let count = card.mine.iter()
        .filter(|m| card.winners.contains(m))
        .collect::<Vec<_>>()
        .len();

    if count > 0 {
        2u32.pow(count as u32 - 1)
    } else {
        0
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(cards: &[Card]) -> u32 {
    cards.iter()
        .map(|c| calc_points(c))
        .sum()
}

fn calc_winners(card: &Card) -> u32 {
    card.mine.iter()
        .filter(|m| card.winners.contains(m))
        .collect::<Vec<_>>()
        .len() as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(cards: &[Card]) -> u32 {
    let mut copies: [u32; NUM_CARDS] = [1; NUM_CARDS];
    for card in cards.iter() {
        let winners = calc_winners(card);
        let start = card.id ;
        let end = std::cmp::min(card.id + winners, NUM_CARDS as u32 - 1);
        let current_card_index = (card.id - 1) as usize;
        for i in start..end {
                copies[i as usize] += copies[current_card_index];
        }
    }
    copies.iter().sum()
}
//11827296
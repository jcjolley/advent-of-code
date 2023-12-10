use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use strum_macros::Display;


pub struct Card {
    value: char
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Display)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: u32
}

impl Hand {
    fn from(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap().chars().map(|ch| Card { value: ch }).collect::<Vec<_>>();
        let hand_type = get_hand_type(&cards);
        let bet = parts.next().unwrap().parse::<u32>().unwrap();

        Self {
            cards,
            hand_type,
            bet
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cards: ").unwrap();
        for card in &self.cards {
            write!(f, "{}", card).unwrap();
        }

        write!(f, " | Type: {}", self.hand_type).unwrap();

        write!(f, " | Bet: {}", self.bet).unwrap();
        Ok(())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        get_card_rank(&self).cmp(&get_card_rank(&other))
    }
}

fn get_card_rank(card: &Card) -> u8 {
    match card.value {
        'A'=> 14,
        'K'=> 13,
        'Q'=> 12,
        'J'=> 11,
        'T'=> 10,
        '9'=> 9,
        '8'=> 8,
        '7'=> 7,
        '6'=> 6,
        '5'=> 5,
        '4'=> 4,
        '3'=> 3,
        '2'=> 2,
        c => panic!("Unsupported card value {}", c)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card { }

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let stype = get_hand_type(&self.cards);
        let otype = get_hand_type(&other.cards);
        match stype.cmp(&otype) {
            Ordering::Equal => {
                let mut card_ord: Ordering = Ordering::Equal;
                for i in 0..5 {
                    let scard = self.cards.iter().nth(i).unwrap();
                    let ocard = other.cards.iter().nth(i).unwrap();
                    card_ord = scard.cmp(ocard);
                    if card_ord != Ordering::Equal {
                        break
                    }
                }
                card_ord
            }
            s => s
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}


fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let seen = build_cards_seen(cards);
    match seen {
        s if s.len() == 1 => HandType::FiveOfAKind,
        s if s.len() == 2 && s.values().max().unwrap() == &4u8 => HandType::FourOfAKind,
        s if s.len() == 2 && s.values().max().unwrap() == &3u8 => HandType::FullHouse,
        s if s.len() == 3 && s.values().max().unwrap() == &3u8 => HandType::ThreeOfAKind,
        s if s.len() == 3 && s.values().max().unwrap() == &2u8 => HandType::TwoPair,
        s if s.len() == 4 => HandType::OnePair,
        s if s.len() == 5 => HandType::HighCard,
        _ => panic!("Unsupported cards")
    }
}

fn build_cards_seen(cards: &Vec<Card>) -> HashMap<char, u8> {
    let mut seen : HashMap<char, u8> = HashMap::new();
    for card in cards {
        seen.entry(card.value).and_modify(|v| *v += 1).or_insert(1);
    }
    seen
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut hands = input.lines().map(|line| Hand::from(line)).collect::<Vec<_>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bet).sum()
}
#[aoc(day7, part2)]
pub fn solve_part2(_input: &str) -> usize {
    return 0
}

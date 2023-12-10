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
        'T'=> 10,
        '9'=> 9,
        '8'=> 8,
        '7'=> 7,
        '6'=> 6,
        '5'=> 5,
        '4'=> 4,
        '3'=> 3,
        '2'=> 2,
        'J'=> 1, // Worthless jokers messing up my pretty code!
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
        s if s.len() == 1 || s.len() == 2 && s.get(&'J').is_some() => HandType::FiveOfAKind,
        s if is_four_of_a_kind(&s) => HandType::FourOfAKind,
        s if is_full_house(&s) => HandType::FullHouse,
        s if is_three_of_a_kind(&s) => HandType::ThreeOfAKind,
        s if is_two_pair(&s) => HandType::TwoPair,
        s if is_one_pair(&s) => HandType::OnePair,
        s if s.len() == 5 => HandType::HighCard,
        _ => {
            for card in cards {
                print!("{}", card)
            }
            panic!("Unsupported cards")
        }
    }
}

fn is_four_of_a_kind(s: &HashMap<char, u8>) -> bool {
    let num_jokers = s.get(&'J');
    if num_jokers.is_none() {
        s.len() == 2 && s.values().max().unwrap() == &4u8
    } else {
        let mut sm = s.clone();
        sm.remove(&'J');
        sm.len() == 2 && sm.values().max().unwrap() + num_jokers.unwrap() == 4u8
    }
}

fn is_full_house(s: &HashMap<char, u8>) -> bool {
    let num_jokers = s.get(&'J');
    if num_jokers.is_none() {
        s.len() == 2 && s.values().max().unwrap() == &3u8
    } else {
        let mut sm = s.clone();
        sm.remove(&'J');
        sm.len() == 2 && sm.values().max().unwrap() + num_jokers.unwrap() == 3u8
    }
}

fn is_three_of_a_kind(s: &HashMap<char, u8>) -> bool {
    let num_jokers = s.get(&'J');
    if num_jokers.is_none() {
        s.len() == 3 && s.values().max().unwrap() == &3u8
    } else {
        let mut sm = s.clone();
        sm.remove(&'J');
        sm.len() == 3 && sm.values().max().unwrap() + num_jokers.unwrap() == 3u8
    }
}
fn is_two_pair(s: &HashMap<char, u8>) -> bool {
    let num_jokers = s.get(&'J');
    if num_jokers.is_none() {
        s.len() == 3 && s.values().max().unwrap() == &2u8
    } else {
        let mut sm = s.clone();
        sm.remove(&'J');
        sm.len() == 3 && sm.values().max().unwrap() + num_jokers.unwrap() == 2u8
    }
}

fn is_one_pair(s: &HashMap<char, u8>) -> bool {
    let num_jokers = s.get(&'J');
    if num_jokers.is_none() {
        s.len() == 4
    } else {
        let mut sm = s.clone();
        sm.remove(&'J');
        sm.len() == 4
    }
}



fn build_cards_seen(cards: &Vec<Card>) -> HashMap<char, u8> {
    let mut seen : HashMap<char, u8> = HashMap::new();
    for card in cards {
        seen.entry(card.value).and_modify(|v| *v += 1).or_insert(1);
    }
    seen
}

#[aoc(day7, part2)]
pub fn solve_part1(input: &str) -> u32 {
    let mut hands = input.lines().map(|line| Hand::from(line)).collect::<Vec<_>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bet).sum()
}

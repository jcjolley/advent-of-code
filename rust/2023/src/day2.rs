use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(EnumIter, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

struct Pull {
    color: Color,
    count: u8
}

impl Pull {
    fn from(input: &str) -> Self  {
        match input {
            r if r.contains("red") => Self {
                color: Color::Red,
                count: r.split_whitespace().nth(0).unwrap().parse::<u8>().unwrap()
            },
            g if g.contains("green") => Self {
                color: Color::Green,
                count: g.split_whitespace().nth(0).unwrap().parse::<u8>().unwrap()
            },
            b if b.contains("blue") => Self {
                color: Color::Blue,
                count: b.split_whitespace().nth(0).unwrap().parse::<u8>().unwrap()
            },
            _ => panic!("Unexpected input")
        }
    }
}

type Turn = Vec<Pull>;

pub struct Game {
    id: u8,
    turns: Vec<Turn>
}

impl Game {
    fn from(input: &str) -> Self {
        let mut parts= input.split(':');
        let id = parts.next().unwrap()[5..].parse::<u8>().unwrap();
        let turns= parts.next()
            .unwrap()
            .split(';')
            .map(|t: &str| { t.split(',').map(|p|{ Pull::from(p) }).collect()
            }).collect();
        Self {
            id,
            turns
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map (|l| Game::from(l)).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(games: &[Game]) -> u16 {
    games.iter()
        .filter(|g| is_valid_game(g))
        .map(|g| g.id as u16)
        .sum()
}

fn is_valid_game(game: &Game) -> bool {
    game.turns.iter().all(|t| {
        t.iter().all(|p| {
            match p.color {
               Color::Red => p.count <= 12,
               Color::Green => p.count <= 13,
               Color::Blue => p.count <= 14
            }
        })
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(games: &[Game]) -> u32 {
    games.iter()
        .map(|g| calculate_power(g))
        .sum()
}

fn calculate_power(game: &Game) -> u32 {
    let pulls = game.turns.iter().flatten().collect();
    Color::iter()
        .map (|c| max_by_color(&pulls, c))
        .reduce(|a, x| a * x)
        .unwrap()
}

fn max_by_color(pulls: &Vec<&Pull>, color: Color) -> u32 {
    pulls.iter()
        .filter(|p| p.color == color)
        .max_by(|p1, p2| p1.count.cmp(&p2.count))
        .unwrap()
        .count as u32
}

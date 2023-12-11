use std::collections::HashMap;
use gcd::Gcd;
enum Direction {
    Left,
    Right
}

pub struct Maze {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Maze { let mut lines = input.lines();
    let directions_str = lines.next().unwrap();
    let directions = directions_str
        .chars()
        .map(|ch|
            match ch {
                'L' => Direction::Left,
                'R' => Direction::Right,
                c => panic!("Unsupported direction character: {}", c)
            }
        )
        .collect::<Vec<_>>();

    // consume the empty line
    lines.next();

    let mut nodes = HashMap::new();
    lines.for_each(|l| {
        let mut parts = l.split('=');
        let value = parts.next().unwrap().trim().to_string();
        let mut child_parts = parts.next().unwrap().split(',');
        let left = child_parts.next().unwrap()[2..5].to_string();
        let right = child_parts.next().unwrap()[1..4].to_string();
        nodes.insert(value, (left, right));
    });

    Maze {
        directions,
        nodes,
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(maze: &Maze) -> u32 {
    let mut steps: u32 = 0;
    let mut node = "AAA";
    let mut directions = maze.directions.iter();

    while node != "ZZZ" {
        let direction = match directions.next() {
            Some(d) => d,
            None => {
                directions = maze.directions.iter();
                directions.next().unwrap()
            }
        };

        node = match direction {
            Direction::Left => &maze.nodes.get(node).unwrap().0,
            Direction::Right => &maze.nodes.get(node).unwrap().1
        };

        steps += 1;
    }

    return steps;
}


#[aoc(day8, part2)]
pub fn solve_part2(maze: &Maze) -> u64 {
    let mut steps: Vec<u64> = Vec::new();
    let nodes = maze
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect::<Vec<_>>();

    let mut directions = maze.directions.iter();

    for mut node in nodes.iter() {
        let mut step: u64 = 0;
        while !node.ends_with('Z') {
            let direction = match directions.next() {
                Some(d) => d,
                None => {
                    directions = maze.directions.iter();
                    directions.next().unwrap()
                }
            };

            node = match direction {
                Direction::Left => &maze.nodes.get(node).unwrap().0,
                Direction::Right => &maze.nodes.get(node).unwrap().1
            };

            step += 1;
        }
        steps.push(step);
    }
    return steps.into_iter().reduce(|a, x| a * x / x.gcd(a.clone())).unwrap().clone();

}
use std::collections::BTreeMap;

pub struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<BTreeMap<u32, [u32; 2]>>
}

impl Almanac {
    fn traverse (&self, seed: &u32) -> u32 {
        let mut val = seed.clone();

        for i in 0..self.maps.len() {
            // Find the highest key below val and see if we're within the offset
            val = match self.maps[i].range(0..=val).next_back() {
                Some(x) => {
                    let source_start = x.0;
                    let [dest_start, offset] = x.1;
                    let diff = val - source_start;
                    if diff < *offset {
                        // We have a mapping!
                        dest_start + diff
                    } else {
                        // No mapping here :(
                        val
                    }

                },
                None => val
            }
        }

        return val;
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    let mut maps = Vec::new();
    let mut lines = input.lines();
    let seeds = lines.next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // drop the empty line and first map title
    lines.next();
    lines.next();

    let mut map: BTreeMap<u32, [u32;2]> = BTreeMap::new();
    let mut map_i = 0;

    lines.for_each(|line: &str| {
        let first_char = line.chars().nth(0);
        match first_char {
            Some(ch) => {
                // If it's a number, it's part of the next set
                match ch {
                    c if c.is_ascii_digit() => {
                        let mut parts = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
                        let dest_start = parts.next().unwrap();
                        let source_start = parts.next().unwrap();
                        let range_size = parts.next().unwrap();

                        map.insert(source_start, [dest_start, range_size]);
                    }
                    // Anything else we don't care about
                    _ => ()
                }
            }
            // Empty lines signify the end of input
            None => {
                maps.insert(map_i, map.clone());
                map_i += 1;
                map = BTreeMap::new()
            }
        };
    });

    // Add the last map
    maps.insert(map_i, map.clone());

    Almanac {
        seeds,
        maps
    }

}

#[aoc(day5, part1)]
pub fn solve_part1(almanac: &Almanac) -> u32 {
    almanac.seeds.iter().map(|seed| almanac.traverse(seed)).min().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(almanac: &Almanac) -> u32 {
   almanac
       .seeds
       .chunks_exact(2)
       .flat_map(|x| x[0]..(x[0] + x[1]))
       .map(|seed| almanac.traverse(&seed)).min().unwrap()
}


use std::cmp::Ordering;
use std::ops::Range;
use std::collections::HashSet;
pub struct Schematic {
    chars: Vec<Vec<char>>
}

impl Schematic {
    fn from(input: &str) -> Self {
        let chars = input.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self {
            chars
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        let max_y = self.chars.len();
        let max_x = self.chars[0].len();
        if x < max_x && y < max_y {
            self.chars[y][x]
        } else {
            '.'
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Schematic {
    Schematic::from(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(schematic: &Schematic) -> u32 {
    let mut total: u32 = 0;
    for y in 0..140 {
        let mut x_range: Range<usize> = 0..140;
        while let Some(x) = x_range.next() {
            let c = schematic.get(x, y);
            if c == '*' {
                let mut nx = x.clone();



                while schematic.get(nx, y).is_ascii_digit() {
                    nx += 1
                }

                if (x..nx).any(|it| search_for_symbol(schematic, &it, &y)) {
                    for _ in x..nx { x_range.next(); }
                    let num: u32 = (x..nx).map(|it| schematic.get(it, y)).collect::<String>().parse().unwrap();
                    total += num;
                }
            }
        }
    }
    total
}

fn get_adjacent_indices(x: &usize, y:&usize) -> [(usize, usize); 8] {
    return [
        (*x - 1, *y + 1), (*x, *y + 1), (*x + 1, *y + 1),
        (*x - 1, *y), (*x + 1, *y),
        (*x - 1, *y - 1), (*x, *y - 1), (*x + 1, *y - 1)
    ]
}

fn search_for_symbol(schematic: &Schematic, x: &usize, y: &usize) -> bool {
   get_adjacent_indices(x, y).into_iter().any(|(nx, ny)|
       match schematic.get(nx, ny) {
           '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => false,
           _ => true
       }
   )
}


fn search_for_non_contiguous_numbers(schematic: &Schematic, x: &usize, y: &usize) -> Vec<(usize, usize)> {
    let npoints = get_adjacent_indices(x, y).into_iter().filter(|(nx, ny)| schematic.get(*nx,*ny).is_ascii_digit()).collect::<Vec<_>>();
    let mut nums = Vec::new();
    'outer: for (x1, y1) in npoints.iter() {
        for (x2, y2) in npoints.iter() {
            if (y1.cmp(y2) != Ordering::Equal) {
                nums.push((*x1, *y1));
                nums.push((*x2, *y2));
                break 'outer
            }

            // todo: if there are two numbers on the same line, check if there's a space between them
            // like 7.2
            //      .*.
            //      ...
        }
    }

    return nums
}

#[aoc(day3, part2)]
pub fn solve_part2(schematic: &Schematic) -> u32 {
    let mut total: u32 = 0;
    for y in 0..140 {
        let mut x_range: Range<usize> = 0..140;
        while let Some(x) = x_range.next() {
            let c = schematic.get(x, y);
            if c == '*' {
                let npoints = search_for_non_contiguous_numbers(schematic, &x, &y).collect::<HashSet<(usize, usize)>>();
                if (npoints.len() == 2)
                {
                    //todo parse numbers, multiply them, and add them to the total
                }
            }
        }
    }
    total
}


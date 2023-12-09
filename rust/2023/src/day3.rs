use std::cmp::Ordering;
use std::ops::Range;
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
    for y in 0..schematic.chars.len() {
        let mut x_range: Range<usize> = 0..schematic.chars[0].len();
        while let Some(x) = x_range.next() {
            let c = schematic.get(x, y);
            // Find a number
            if c.is_ascii_digit() {
                let mut nx = x.clone();

                // Find all the digits in the number
                while schematic.get(nx, y).is_ascii_digit() {
                    nx += 1
                }

                // See if there's a symbol touching the number
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
           c if c.is_ascii_digit() || c == '.' => false,
           _ => true
       }
   )
}


fn search_for_non_contiguous_numbers(schematic: &Schematic, x: &usize, y: &usize) -> Vec<(usize, usize)> {
    let npoints = get_adjacent_indices(x, y).into_iter().filter(|(nx, ny)| schematic.get(*nx,*ny).is_ascii_digit()).collect::<Vec<_>>();
    let mut nums = Vec::new();
    'outer: for (x1, y1) in npoints.iter() {
        for (x2, y2) in npoints.iter() {
            let low_x = std::cmp::min(*x1, *x2);
            let has_gap = ((*x1 as i8) - (*x2 as i8)).abs() == 2 && !schematic.get(low_x + 1, *y1).is_ascii_digit();
            // If they are on different lines, they're not the same number.  If they're on the same line, there's gotta be a gap
            if y1.cmp(y2) != Ordering::Equal || has_gap {
                nums.push((*x1, *y1));
                nums.push((*x2, *y2));
                break 'outer
            }
        }
    }
    return nums;
}

#[aoc(day3, part2)]
pub fn solve_part2(schematic: &Schematic) -> u32 {
    let mut total: u32 = 0;
    for y in 0..140 {
        let mut x_range: Range<usize> = 0..140;
        while let Some(x) = x_range.next() {
            let c = schematic.get(x, y);
            // Find a gear
            if c == '*' {
                // Find all the numbers touching the touching the gear
                let npoints = search_for_non_contiguous_numbers(schematic, &x, &y);

                // When there are adjacent numbers to a gear, multiply them and add to total
                if npoints.len() == 2 {
                    total += npoints.iter().map(|(x, y)| {
                        if y > &schematic.chars.len() || x > &schematic.chars[0].len() {
                            0u32
                        } else {
                            extract_number(schematic, x, y)
                        }
                    }).reduce(|a, x| a * x).unwrap();
                }
            }
        }
    }
    total
}

fn extract_number(schematic: &Schematic, x: &usize, y: &usize) -> u32 {
    let mut sx = x.clone();
    while schematic.get(sx,*y).is_ascii_digit() {
        sx -= 1
    }

    let mut ex = x.clone();
    while schematic.get(ex,*y).is_ascii_digit() {
        ex += 1
    }


    let num = ((sx + 1)..ex).map(|it| schematic.get(it, *y)).collect::<String>().parse().unwrap();
    num
}


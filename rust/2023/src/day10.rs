pub struct Network {
    chars: Vec<Vec<char>>,
    bound_x: usize,
    bound_y: usize,
}

impl Network {
    fn from(input: &str) -> Self {
        let mut my_chars = input.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        my_chars.reverse();

        Self {
            chars: my_chars.clone(),
            bound_x: my_chars.clone()[0].len(),
            bound_y: my_chars.clone().len()
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        assert!(x < self.bound_x, "Tried to get ({x}, {y}) is out of bounds {}", self.bound_x);
        assert!(y < self.bound_y, "Tried to get ({x}, {y}) is out of bounds {}", self.bound_y);
        self.chars[y][x]
    }

    fn navigate_clockwise(&self, prev_x: usize, prev_y: usize, x: usize, y: usize) -> (char, usize, usize, usize, usize) {
        let c = self.get(x, y);
        let (nx, ny) = match c {
            // If I came from the south, go north, If I came from the north, go south
            '|' => if (y - prev_y) == 1 { (x, y + 1) } else { (x, y - 1) },
            // If I came from the east, go up.  If I came from the north, go east
            'L' => if (prev_x - x) == 1 { (x, y + 1) } else { (x + 1, y) },
            // If I came from the east, go west, If I came from the west, go east
            '-' => if (prev_x - x) == 1 { (x - 1, y) } else { (x + 1, y) },
            // If I came from the east, go south.  If I came from the south, go east.
            'F' => if (prev_x -x ) == 1 { (x, y - 1) } else { (x + 1, y) }
            // If I came from the west, go north.  If I came from the north, go west
            'J' => if (x - prev_x) == 1 { (x, y + 1) } else { (x - 1, y) },
            // If I came from the west, go south. If I came from the south, go west
            '7' => if (x - prev_x) == 1 { (x, y - 1) } else { (x - 1, y)},
            'S' => (x, y), // We made it!
            d => panic!("Invalid character {d} at ({x}, {y})")
        };

        assert!(nx < self.bound_x, "Tried to navigate to ({nx}, {ny}), which is out of bounds {}", self.bound_x);
        assert!(ny < self.bound_y, "Tried to navigate to ({nx}, {ny}), which is out of bounds {}", self.bound_y);
        println!("Navigating from {c} ({x}, {y}) to ({nx}, {ny})");
        (c, x, y, nx, ny)
    }
}



#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Network{
    Network::from(input)
}

#[aoc(day10, part1)]
pub fn solve_part1(network: &Network) -> i32 {
    let mut prev_x: usize = 0;
    let mut prev_y: usize = 0;
    let mut x = 0;
    let mut y = 0;
    'outer: for start_y in 0..network.chars.len() {
        for start_x in 0..network.chars[0].len() {
            if network.get(start_x,start_y) == 'S' {
                x = start_x;
                y = start_y;
                break 'outer;
            }
        }
    }

    let mut icon = '.';
    let mut count = 0;

    // Take the first step to prime the pump
    let (nx, ny) = {
        let west = network.get(x - 1, y);
        let north = network.get(x, y + 1);
        let east = network.get(x + 1, y);
        let south = network.get(x, y - 1);
        if west == 'L' || west == '-' || west == 'F' {
            (x - 1, y)
        } else if north == '|' || north == 'F' || north == '7' {
            (x, y + 1)
        } else if east == 'J' || east == '7' || east == '-' {
            (x + 1, y)
        } else if south == '|' || south == 'J' || south == 'L' {
            (x, y - 1)
        } else {
            panic!("No way to exit S clockwise")
        }
    };

    (icon, prev_x, prev_y, x, y) = network.navigate_clockwise(x, y, nx, ny);

    while icon != 'S' {
        (icon, prev_x, prev_y, x, y) = network.navigate_clockwise(prev_x, prev_y, x, y);

        count += 1;
    }

    return count / 2 + 1
}



#[aoc(day10, part2)]
pub fn solve_part2(_network: &Network) -> i32 {
    return 0
}



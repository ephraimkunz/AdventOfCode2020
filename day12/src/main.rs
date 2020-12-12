fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut boat = Boat::new();
    let mut waypoint = Waypoint::new();
    for line in input.lines() {
        waypoint.travel(line);
        boat.travel(line, &waypoint);
    }

    println!("{}", boat.manhattan())
}

#[derive(Copy, Clone)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    fn rotate(&self, dir: char, amount: i32) -> Self {
        let (amount, dir) = match (amount, dir) {
            (270, 'L') => (90, 'R'),
            (270, 'R') => (90, 'L'),
            (a, d) => (a, d),
        };

        match (amount, dir) {
            (90, 'L') => match self {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
            },
            (90, 'R') => match self {
                Direction::East => Direction::South,
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
            },
            (180, _) => match self {
                Direction::East => Direction::West,
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
            },
            _ => unreachable!(),
        }
    }

    fn travel(&self, amount: i32, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::East => (x + amount, y),
            Direction::West => (x - amount, y),
            Direction::North => (x, y + amount),
            Direction::South => (x, y - amount),
        }
    }
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Self { x: 10, y: 1 }
    }

    fn travel(&mut self, s: &str) {
        let command = s.chars().nth(0).unwrap();
        let arg: String = s.chars().skip(1).collect();
        let arg = arg.parse::<i32>().unwrap();

        match command {
            'N' => self.y += arg,
            'S' => self.y -= arg,
            'E' => self.x += arg,
            'W' => self.x -= arg,
            'L' => match arg {
                90 => {
                    let new_x = -self.y;
                    let new_y = self.x;
                    self.x = new_x;
                    self.y = new_y;
                }
                180 => {
                    self.x = -self.x;
                    self.y = -self.y;
                }
                270 => {
                    let new_x = self.y;
                    let new_y = -self.x;
                    self.x = new_x;
                    self.y = new_y;
                }
                _ => unreachable!(),
            },
            'R' => match arg {
                90 => {
                    let new_x = self.y;
                    let new_y = -self.x;
                    self.x = new_x;
                    self.y = new_y;
                }
                180 => {
                    self.x = -self.x;
                    self.y = -self.y;
                }
                270 => {
                    let new_x = -self.y;
                    let new_y = self.x;
                    self.x = new_x;
                    self.y = new_y;
                }
                _ => unreachable!(),
            },
            _ => {}
        };
    }
}

struct Boat {
    x: i32,
    y: i32,
}

impl Boat {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn travel(&mut self, s: &str, waypoint: &Waypoint) {
        let command = s.chars().nth(0).unwrap();
        let arg: String = s.chars().skip(1).collect();
        let arg = arg.parse::<i32>().unwrap();

        match command {
            'F' => {
                for _ in 0..arg {
                    self.x += waypoint.x;
                    self.y += waypoint.y;
                }
            }
            _ => {}
        };
    }

    fn manhattan(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

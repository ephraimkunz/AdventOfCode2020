use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let num_valid = input
        .lines()
        .map(|l| l.parse::<Entry>().unwrap())
        .fold(0, |accum, e| {
            if e.is_valid_part_b() {
                accum + 1
            } else {
                accum
            }
        });
    println!("{}", num_valid);
}

struct Entry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Entry {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().fold(
            0,
            |accum, c| if c == self.letter { accum + 1 } else { accum },
        );
        count >= self.min && count <= self.max
    }

    fn is_valid_part_b(&self) -> bool {
        match (
            self.password.chars().nth(self.min - 1).unwrap() == self.letter,
            self.password.chars().nth(self.max - 1).unwrap() == self.letter,
        ) {
            (true, false) | (false, true) => true,
            _ => false,
        }
    }
}

impl FromStr for Entry {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let reg = regex::Regex::new(
            r#"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)$"#,
        )
        .unwrap();
        let caps = reg.captures(s).unwrap();
        Ok(Self {
            min: caps["min"].parse()?,
            max: caps["max"].parse()?,
            letter: caps["letter"].chars().nth(0).unwrap(),
            password: caps["password"].to_string(),
        })
    }
}

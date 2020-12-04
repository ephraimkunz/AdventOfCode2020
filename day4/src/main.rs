// This works, but it's pretty messy. Is there a better way without having to pull in regex?

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut num_good = 0;

    let mut record = Record::new();
    for line in input.lines() {
        if !line.contains(':') {
            if record.is_valid() {
                num_good += 1
            }
            record = Record::new();
        } else {
            for field in line.split_whitespace() {
                let kv: Vec<_> = field.splitn(2, ':').collect();
                match kv[0] {
                    "byr" => {
                        let date: u32 = kv[1].parse().unwrap();
                        record.byr = kv[1].len() == 4 && date >= 1920 && date <= 2002
                    }
                    "iyr" => {
                        let date: u32 = kv[1].parse().unwrap();
                        record.iyr = kv[1].len() == 4 && date >= 2010 && date <= 2020
                    }
                    "eyr" => {
                        let date: u32 = kv[1].parse().unwrap();
                        record.eyr = kv[1].len() == 4 && date >= 2020 && date <= 2030
                    }
                    "pid" => {
                        let num = kv[1].parse::<u64>();
                        record.pid = num.is_ok() && kv[1].len() == 9
                    }
                    "hgt" => {
                        if kv[1].ends_with("cm") {
                            if let Ok(num) = kv[1].split_at(kv[1].len() - 2).0.parse::<u32>() {
                                record.hgt = num >= 150 && num <= 193;
                            }
                        } else if kv[1].ends_with("in") {
                            if let Ok(num) = kv[1].split_at(kv[1].len() - 2).0.parse::<u32>() {
                                record.hgt = num >= 59 && num <= 76;
                            }
                        }
                    }
                    "hcl" => {
                        if kv[1].len() == 7 && kv[1].chars().nth(0).unwrap() == '#' {
                            let mut good = true;
                            for c in kv[1].chars().skip(1) {
                                match c {
                                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                                    | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' => {}
                                    _ => {
                                        good = false;
                                        break;
                                    }
                                }
                            }
                            if good {
                                record.hcl = true
                            }
                        }
                    }
                    "cid" => record.cid = true,
                    "ecl" => match kv[1] {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => record.ecl = true,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    if record.is_valid() {
        num_good += 1;
    }

    println!("{}", num_good);
}

struct Record {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Record {
    fn is_valid(&self) -> bool {
        return self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid;
    }

    fn new() -> Record {
        Record {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }
}

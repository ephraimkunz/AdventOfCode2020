fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let instructions: Vec<_> = input
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
                false,
            )
        })
        .collect();
    part1(instructions.clone());
    part2(instructions);
}

fn part1(mut instructions: Vec<(&str, i32, bool)>) {
    let mut accum = 0;
    let mut ip: isize = 0;
    loop {
        let i = instructions.iter_mut().nth(ip as usize).unwrap();
        if i.2 {
            break;
        }
        match i.0 {
            "jmp" => {
                ip += i.1 as isize;
            }
            "acc" => {
                accum += i.1;
                ip += 1;
            }
            _ => {
                ip += 1;
            }
        };
        i.2 = true;
    }

    println!("{}", accum);
}

fn part2(instructions: Vec<(&str, i32, bool)>) {
    for i in 0..instructions.len() {
        let mut cloned_is = instructions.clone();
        let instruction = cloned_is.iter_mut().nth(i).unwrap();
        match instruction.0 {
            "jmp" => {
                instruction.0 = "nop";
            }
            "nop" => {
                instruction.0 = "jmp";
            }
            _ => {
                continue;
            }
        };

        match run(cloned_is) {
            Some(a) => {
                println!("{}", a);
                return;
            }
            _ => (),
        };
    }
}

fn run(mut instructions: Vec<(&str, i32, bool)>) -> Option<i32> {
    let mut accum = 0;
    let mut ip: isize = 0;
    loop {
        if (ip as usize) == instructions.len() {
            return Some(accum);
        }

        let i = instructions.iter_mut().nth(ip as usize).unwrap();
        if i.2 {
            return None;
        }
        match i.0 {
            "jmp" => {
                ip += i.1 as isize;
            }
            "acc" => {
                accum += i.1;
                ip += 1;
            }
            _ => {
                ip += 1;
            }
        };
        i.2 = true;
    }
}

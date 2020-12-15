use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut turn = 1;
    let mut last_spoken = 0;
    for number in input.split(",").map(|n| n.parse::<u32>().unwrap()) {
        map.entry(number).or_insert(vec![]).push(turn);
        turn += 1;
        last_spoken = number;
    }

    while turn <= 30000000 {
        if let Some(val) = map.get(&last_spoken) {
            if val.len() == 1 {
                // First time spoken.
                map.entry(0).or_insert(vec![]).push(turn);
                turn += 1;
                last_spoken = 0;
            } else {
                // Spoken multiple times previously.
                let mut iter = val.iter();
                let num = iter.next_back().unwrap() - iter.next_back().unwrap();
                map.entry(num).or_insert(vec![]).push(turn);
                turn += 1;
                last_spoken = num;
            }
        } else {
            // First time spoken.
            map.entry(0).or_insert(vec![]).push(turn);
            turn += 1;
            last_spoken = 0;
        }
    }

    println!("{}", last_spoken);
}

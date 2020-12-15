fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let start: u32 = input.lines().next().unwrap().parse().unwrap();
    let bus_ids: Vec<_> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .filter_map(|id| id.parse::<u32>().ok())
        .collect();
    part1(start, &bus_ids);

    let bus_ids: Vec<_> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, id)| Some((i, id.parse::<usize>().ok()?)))
        .collect();
    part2(&bus_ids);
}

fn part1(start: u32, bus_ids: &[u32]) {
    let mut current_time = start;
    loop {
        for id in bus_ids {
            if current_time % id == 0 {
                println!("{}", (current_time - start) * id);
                return;
            }
        }
        current_time += 1;
    }
}

// Chinese remainder theorm.
fn part2(bus_ids: &[(usize, usize)]) {
    let modulii = bus_ids
        .iter()
        .map(|id| id.1 as i64)
        .collect::<Vec<_>>();
    let residues = bus_ids
        .iter()
        .map(|id| id.1 as i64 - id.0 as i64)
        .collect::<Vec<_>>();

    match chinese_remainder(&residues, &modulii) {
        Some(sol) => println!("{}", sol),
        None => println!("modulii not pairwise coprime"),
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

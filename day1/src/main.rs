use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let numbers: Vec<u32> = input.lines().map(|i| i.parse().unwrap()).collect();
    let found = numbers
        .iter()
        .permutations(2)
        .find(|v| v.iter().map(|&&x| x).sum::<u32>() == 2020)
        .unwrap();
    println!("{}", found.iter().map(|&&x| x).product::<u32>());
}

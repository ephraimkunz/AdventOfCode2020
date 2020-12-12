use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let numbers: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    part1(&numbers);
    part2(&numbers);
}

fn part1(input: &Vec<u32>) {
    let mut input = input.clone();
    input.push(0);
    input.sort();
    input.push(input.iter().last().unwrap() + 3);

    let diffs = input.windows(2).map(|w| w[1] - w[0]);
    let mut one_diff = 0;
    let mut three_diff = 0;
    for diff in diffs {
        if diff == 1 {
            one_diff += 1;
        } else if diff == 3 {
            three_diff += 1;
        }
    }
    println!("{}", one_diff * three_diff);
}

fn part2(input: &Vec<u32>) {
    let mut input = input.clone();
    input.push(0);
    input.sort();
    input.push(input.iter().last().unwrap() + 3);

    let mut map: HashMap<usize, u64> = HashMap::new();
    println!("{}", dp(0, &input, &mut map));
}

fn dp(i: usize, input: &Vec<u32>, map: &mut HashMap<usize, u64>) -> u64 {
    if i == (input.len() - 1) {
        return 1;
    }

    if map.contains_key(&i) {
        return map[&i];
    }

    let mut ans = 0;
    for j in (i + 1)..(input.len() as usize) {
        if (input[j] - input[i]) <= 3 {
            ans += dp(j, input, map);
        }
    }

    map.insert(i, ans);

    return ans;
}

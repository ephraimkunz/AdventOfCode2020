use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let groups = input.split("\n\n").map(|s| s.replace("\n", ""));
    let unique_answers = groups.map(|g| {
        let answers: HashSet<_> = g.chars().collect();
        answers.len()
    });

    println!("{}", unique_answers.sum::<usize>());
}

fn part2(input: &str) {
    let groups = input.split("\n\n");

    let group_sets = groups.map(|s| {
        s.split('\n')
            .map(|p| {
                let answers: HashSet<_> = p.chars().collect();
                answers
            })
            .collect::<Vec<HashSet<_>>>()
    });

    let group_counts = group_sets.map(|sets| {
        let combined_set: HashSet<_> = sets[0]
            .iter()
            .filter(|b| sets[1..].iter().all(|set| set.contains(*b)))
            .collect();
        combined_set.len()
    });

    println!("{}", group_counts.sum::<usize>());
}

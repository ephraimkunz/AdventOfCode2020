use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut bag_name_to_id = HashMap::new();
    for (index, line) in input.lines().enumerate() {
        let mut iter = line.splitn(2, "s contain ");

        let name = iter.next().unwrap();
        bag_name_to_id.insert(name, index);
    }

    let mut next_id = input.lines().count();

    let mut child_to_parents: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.splitn(2, "s contain ");

        let parent_name = iter.next().unwrap();
        let parent_id = bag_name_to_id[parent_name];

        let contains = iter.next().unwrap();
        for child in contains.split(", ") {
            let mut c = child;
            if child.ends_with('s') {
                c = &child[..(child.len() - 1)];
            } else if child.ends_with("s.") {
                c = &child[..(child.len() - 2)];
            } else if child.ends_with('.') {
                c = &child[..(child.len() - 1)];
            }

            let mut child_iter = c.splitn(2, " ");
            let count = child_iter.next().unwrap();
            let c_name = child_iter.next().unwrap();
            let c_id = if bag_name_to_id.contains_key(c_name) {
                bag_name_to_id[c_name]
            } else {
                bag_name_to_id.insert(c_name, next_id);
                next_id += 1;
                next_id - 1
            };
            child_to_parents
                .entry(c_id)
                .or_insert(vec![])
                .push(parent_id);
        }
    }

    let mut working_set = HashSet::new();
    let mut shiny_gold_parents = HashSet::new();
    let shiny_gold_id = bag_name_to_id["shiny gold bag"];

    working_set.insert(shiny_gold_id);
    while !working_set.is_empty() {
        let id = working_set.iter().next().unwrap().clone();
        if child_to_parents.contains_key(&id) {
            let parents = &child_to_parents[&id];

            for &p in parents {
                shiny_gold_parents.insert(p);
                working_set.insert(p);
            }
        }
        working_set.remove(&id);
    }

    println!("{:?}", shiny_gold_parents.len());
}

fn part2(input: &str) {
    let mut bag_name_to_id = HashMap::new();
    for (index, line) in input.lines().enumerate() {
        let mut iter = line.splitn(2, "s contain ");

        let name = iter.next().unwrap();
        bag_name_to_id.insert(name, index);
    }

    let mut next_id = input.lines().count();

    let mut parent_to_children: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.splitn(2, "s contain ");

        let parent_name = iter.next().unwrap();
        let parent_id = bag_name_to_id[parent_name];

        let contains = iter.next().unwrap();
        for child in contains.split(", ") {
            let mut c = child;
            if child.ends_with('s') {
                c = &child[..(child.len() - 1)];
            } else if child.ends_with("s.") {
                c = &child[..(child.len() - 2)];
            } else if child.ends_with('.') {
                c = &child[..(child.len() - 1)];
            }

            if c == "no other bag" {
                parent_to_children.entry(parent_id).or_insert(vec![]);
                continue;
            }

            let mut child_iter = c.splitn(2, " ");
            let count = child_iter.next().unwrap();
            let c_count = count.parse::<usize>().unwrap();
            let c_name = child_iter.next().unwrap();
            let c_id = if bag_name_to_id.contains_key(c_name) {
                bag_name_to_id[c_name]
            } else {
                bag_name_to_id.insert(c_name, next_id);
                next_id += 1;
                next_id - 1
            };
            parent_to_children
                .entry(parent_id)
                .or_insert(vec![])
                .push((c_id, c_count));
        }
    }

    let mut id_to_count = HashMap::new();
    for (k, v) in &parent_to_children {
        if v.is_empty() {
            id_to_count.insert(k, 0);
        }
    }

    while id_to_count.len() != bag_name_to_id.len() {
        for (k, v) in &parent_to_children {
            if v.iter().all(|(idx, _)| id_to_count.contains_key(idx)) {
                let count = v
                    .iter()
                    .map(|(idx, count)| (id_to_count[idx] + 1) * count)
                    .sum();
                id_to_count.insert(k, count);
            }
        }
    }

    println!("{:?}", id_to_count[&bag_name_to_id["shiny gold bag"]]);
}

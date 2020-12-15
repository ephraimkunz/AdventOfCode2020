use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Prog {
    mask: String,
    mems: Vec<(u64, u64)>,
}

impl Prog {
    fn new(mask: String) -> Self {
        Self { mask, mems: vec![] }
    }
}

fn part1(input: &str) {
    let mut progs = vec![];
    for line in input.lines() {
        if line.starts_with("mask") {
            progs.push(Prog::new(line[7..].to_string()));
        } else {
            let open = line.find('[').unwrap();
            let close = line.find(']').unwrap();
            progs.last_mut().unwrap().mems.push((
                line[(open + 1)..close].parse().unwrap(),
                line[(close + 4)..].parse().unwrap(),
            ))
        }
    }

    let max_mem = progs
        .iter()
        .flat_map(|prog| prog.mems.iter())
        .fold(0, |accum, item| accum.max(item.0));
    let mut memory = vec![0; (max_mem + 1) as usize];
    for prog in progs {
        for (addr, val) in prog.mems {
            let mut val_masked = val;
            for i in 0..prog.mask.len() {
                match prog.mask.chars().nth_back(i).unwrap() {
                    '0' => val_masked &= !(1 << i),
                    '1' => val_masked |= 1 << i,
                    _ => {}
                }
            }
            memory[addr as usize] = val_masked;
        }
    }
    println!("{}", memory.iter().sum::<u64>());
}

fn part2(input: &str) {
    let mut progs = vec![];
    for line in input.lines() {
        if line.starts_with("mask") {
            progs.push(Prog::new(line[7..].to_string()));
        } else {
            let open = line.find('[').unwrap();
            let close = line.find(']').unwrap();
            progs.last_mut().unwrap().mems.push((
                line[(open + 1)..close].parse().unwrap(),
                line[(close + 4)..].parse().unwrap(),
            ))
        }
    }

    let mut memory = HashMap::new();

    for prog in progs {
        for (addr, val) in prog.mems {
            let mut addr_masked = prog.mask.clone().into_bytes();
            let mut x_indices = vec![];
            let mut num_x = 0;
            for i in 0..prog.mask.len() {
                match prog.mask.chars().nth_back(i).unwrap() {
                    'X' => {
                        num_x += 1;
                        x_indices.push(i);
                    }
                    '0' => {
                        *addr_masked.iter_mut().nth_back(i).unwrap() =
                            if ((1 << i) & addr) > 0 { b'1' } else { b'0' };
                    }
                    _ => {}
                }
            }

            let mut addresses = vec![];
            let num_addresses = 2u32.pow(num_x);
            if num_addresses == 1 {
                addresses.push(String::from_utf8(addr_masked).unwrap());
            } else {
                for i in 0..num_addresses {
                    let mut addr = addr_masked.clone();
                    for (x_indices_index, &x_indice) in x_indices.iter().enumerate() {
                        *addr.iter_mut().nth_back(x_indice).unwrap() =
                            if ((1 << x_indices_index) & i) > 0 {
                                b'1'
                            } else {
                                b'0'
                            };
                    }

                    addresses.push(String::from_utf8(addr).unwrap());
                }
            }

            for address in addresses {
                let index = usize::from_str_radix(&address, 2).unwrap();
                memory.insert(index, val);
            }
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

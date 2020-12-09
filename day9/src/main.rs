fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &Vec<u64>) {
    for window in numbers.windows(26) {
        let mut found = false;
        let result = window[25];
        for i in 0..25 {
            let element1 = window[i];
            if result <= element1 {
                continue;
            }

            let expected_element2 = result - element1;
            for j in (i + 1)..25 {
                let element2 = window[j];
                if element2 == expected_element2 {
                    // Found it.
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        if !found {
            println!("{}", result);
            return;
        }
    }
}

fn part2(numbers: &Vec<u64>) {
    let mut start = 0;
    let mut count = 2;
    loop {
        while *&numbers[start..(start + count)].iter().sum::<u64>() < 26134589 {
            count += 1;
            if *&numbers[start..(start + count)].iter().sum::<u64>() == 26134589 {
                println!(
                    "{}",
                    **&numbers[start..(start + count)].iter().min().unwrap()
                        + **&numbers[start..(start + count)].iter().max().unwrap()
                );
                return;
            }
        }
        start += 1;
        count = 2;
    }
}

use std::collections::HashSet;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let ids = generate_ids(&input);
    let max_id = ids.iter().max().unwrap();

    let existing_ids: HashSet<_> = ids.iter().collect();
    let mut candidate = 1;
    loop {
        if existing_ids.contains(&(candidate - 1))
            && existing_ids.contains(&(candidate + 1))
            && !existing_ids.contains(&candidate)
        {
            break;
        }

        candidate += 1;
    }

    println!("{}", max_id);
    println!("{}", candidate);
}

fn generate_ids(input: &str) -> Vec<u32> {
    // We are just parsing binary numbers, where 0 and 1 are represented by certain characters.
    input
        .lines()
        .map(|line| {
            let row_str = &line[..7];
            let mut row = 0;
            let mut bitmask = 1 << 7;
            for letter in row_str.chars() {
                bitmask >>= 1;
                if letter == 'B' {
                    row += bitmask;
                }
            }

            let col_str = &line[7..];
            let mut col = 0;
            let mut bitmask = 1 << 3;
            for letter in col_str.chars() {
                bitmask >>= 1;
                if letter == 'R' {
                    col += bitmask;
                }
            }

            row * 8 + col
        })
        .collect()
}

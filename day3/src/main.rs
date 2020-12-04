
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut mult_accum: u32 = 1;
    for slope_increment in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while y < height {
            if grid[y][x % width] {
                trees += 1;
            }

            x += slope_increment.0;
            y += slope_increment.1;
        }
        println!("{} trees", trees);
        mult_accum *= trees;
    }

    println!("{}", mult_accum);
}

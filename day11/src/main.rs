type Grid = Vec<Vec<Cell>>;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let grid: Grid = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match c {
                    b'L' => Cell::Empty,
                    _ => Cell::Floor,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    part1(grid.clone());
    part2(grid.clone());
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Floor,
    Occupied,
    Empty,
}

fn part1(grid: Grid) {
    let mut old_grid = grid.clone();
    let mut new_grid = old_grid.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for r in 0..old_grid.len() {
            for c in 0..old_grid[0].len() {
                let mut occupied_neighbors = 0;
                if r > 0 {
                    occupied_neighbors += if old_grid[r - 1][c] == Cell::Occupied {
                        1
                    } else {
                        0
                    };

                    if c > 0 {
                        occupied_neighbors += if old_grid[r - 1][c - 1] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                    }

                    if c < (old_grid[0].len() - 1) {
                        occupied_neighbors += if old_grid[r - 1][c + 1] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                    }
                }

                if r < (old_grid.len() - 1) {
                    occupied_neighbors += if old_grid[r + 1][c] == Cell::Occupied {
                        1
                    } else {
                        0
                    };

                    if c > 0 {
                        occupied_neighbors += if old_grid[r + 1][c - 1] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                    }

                    if c < (old_grid[0].len() - 1) {
                        occupied_neighbors += if old_grid[r + 1][c + 1] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                    }
                }

                if c < (old_grid[0].len() - 1) {
                    occupied_neighbors += if old_grid[r][c + 1] == Cell::Occupied {
                        1
                    } else {
                        0
                    };
                }

                if c > 0 {
                    occupied_neighbors += if old_grid[r][c - 1] == Cell::Occupied {
                        1
                    } else {
                        0
                    };
                }

                let cell = old_grid[r][c];
                if cell == Cell::Empty && occupied_neighbors == 0 {
                    new_grid[r][c] = Cell::Occupied;
                    changed = true;
                } else if cell == Cell::Occupied && occupied_neighbors >= 4 {
                    new_grid[r][c] = Cell::Empty;
                    changed = true;
                }
            }
        }

        old_grid = new_grid;
        new_grid = old_grid.clone();
    }

    println!(
        "{}",
        old_grid.iter().fold(0, |accum, row| {
            accum
                + row.iter().fold(0, |accum, &element| {
                    accum + if element == Cell::Occupied { 1 } else { 0 }
                })
        })
    );
}

fn part2(grid: Grid) {
    let mut old_grid = grid.clone();
    let mut new_grid = old_grid.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for r in 0..old_grid.len() {
            for c in 0..old_grid[0].len() {
                let mut occupied_neighbors = 0;

                let mut count = 0;
                for c_s in (0..c).rev() {
                    if old_grid[r][c_s] == Cell::Floor {
                        continue;
                    }

                    count = if old_grid[r][c_s] == Cell::Occupied {
                        1
                    } else {
                        0
                    };
                    break;
                }
                occupied_neighbors += count;

                count = 0;
                for c_s in (c + 1)..old_grid[0].len() {
                    if old_grid[r][c_s] == Cell::Floor {
                        continue;
                    }

                    count = if old_grid[r][c_s] == Cell::Occupied {
                        1
                    } else {
                        0
                    };
                    break;
                }
                occupied_neighbors += count;

                count = 0;
                for r_s in (0..r).rev() {
                    if old_grid[r_s][c] == Cell::Floor {
                        continue;
                    }

                    count = if old_grid[r_s][c] == Cell::Occupied {
                        1
                    } else {
                        0
                    };
                    break;
                }
                occupied_neighbors += count;

                count = 0;
                for r_s in (r + 1)..old_grid.len() {
                    if old_grid[r_s][c] == Cell::Floor {
                        continue;
                    }

                    count = if old_grid[r_s][c] == Cell::Occupied {
                        1
                    } else {
                        0
                    };
                    break;
                }
                occupied_neighbors += count;

                count = 0;
                let mut r_s = (r + 1) as isize;
                let mut c_s = (c + 1) as isize;
                while r_s < old_grid.len() as isize && c_s < old_grid[0].len() as isize {
                    if old_grid[r_s as usize][c_s as usize] != Cell::Floor {
                        count = if old_grid[r_s as usize][c_s as usize] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                        break;
                    }
                    r_s += 1;
                    c_s += 1;
                }
                occupied_neighbors += count;

                count = 0;
                r_s = r as isize - 1;
                c_s = c as isize - 1;
                while r_s >= 0 && c_s >= 0 {
                    if old_grid[r_s as usize][c_s as usize] != Cell::Floor {
                        count = if old_grid[r_s as usize][c_s as usize] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                        break;
                    }
                    r_s -= 1;
                    c_s -= 1;
                }
                occupied_neighbors += count;

                count = 0;
                r_s = r as isize - 1;
                c_s = c as isize + 1;
                while r_s >= 0 && c_s < old_grid[0].len() as isize {
                    if old_grid[r_s as usize][c_s as usize] != Cell::Floor {
                        count = if old_grid[r_s as usize][c_s as usize] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                        break;
                    }
                    r_s -= 1;
                    c_s += 1;
                }
                occupied_neighbors += count;

                count = 0;
                r_s = r as isize + 1;
                c_s = c as isize - 1;
                while r_s < old_grid.len() as isize && c_s >= 0 {
                    if old_grid[r_s as usize][c_s as usize] != Cell::Floor {
                        count = if old_grid[r_s as usize][c_s as usize] == Cell::Occupied {
                            1
                        } else {
                            0
                        };
                        break;
                    }
                    r_s += 1;
                    c_s -= 1;
                }
                occupied_neighbors += count;

                let cell = old_grid[r][c];
                if cell == Cell::Empty && occupied_neighbors == 0 {
                    new_grid[r][c] = Cell::Occupied;
                    changed = true;
                } else if cell == Cell::Occupied && occupied_neighbors >= 5 {
                    new_grid[r][c] = Cell::Empty;
                    changed = true;
                }
            }
        }

        old_grid = new_grid;
        new_grid = old_grid.clone();
    }

    println!(
        "{}",
        old_grid.iter().fold(0, |accum, row| {
            accum
                + row.iter().fold(0, |accum, &element| {
                    accum + if element == Cell::Occupied { 1 } else { 0 }
                })
        })
    );
}

use std::thread::sleep;
use std::time::Duration;

const GRID_COLS: i32 = 100;
const GRID_ROWS: i32 = 50;

const ALIVE: char = '*';
const DEAD: char = '.';

fn cell_to_index(mut x: i32, mut y: i32) -> usize {
    if x < 0 {
        // this solves case when x >= GRID_COLS
        x = (-x) % GRID_COLS;
        x = GRID_COLS - x;
    }
    if y < 0 {
        y = (-y) % GRID_ROWS;
        y = GRID_ROWS - y;
    }
    if x >= GRID_COLS {
        x = x % GRID_COLS;
    }
    if y >= GRID_ROWS {
        y = y % GRID_ROWS;
    }

    (y * GRID_COLS + x) as usize
}

struct Grid {
    data: [u8; (GRID_COLS * GRID_ROWS) as usize],
    alive_count: u32,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            data: [DEAD.try_into().unwrap(); (GRID_COLS * GRID_ROWS) as usize],
            alive_count: 0,
        }
    }
    fn set_cell(&mut self, x: i32, y: i32, cell: char) {
        if cell == ALIVE {
            self.alive_count += 1;
        } else {
            if let Some(value) = self.alive_count.checked_sub(1) {
                self.alive_count = value;
            } else {
                self.alive_count = 0;
            }
        }
        self.data[cell_to_index(x, y)] = cell as u8;
    }
    fn get_cell(&self, x: i32, y: i32) -> char {
        self.data[cell_to_index(x, y)] as char
    }

    fn fill_random(&mut self) {
        for y in 0..GRID_ROWS {
            for x in 0..GRID_COLS {
                self.set_cell(x, y, if rand::random_bool(0.4) { ALIVE } else { DEAD });
            }
        }
    }

    fn count_alive_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                if self.get_cell(x + j, y + i) == ALIVE {
                    count += 1;
                };
            }
        }
        count
    }

    fn next_generation(self) -> Grid {
        let mut next_generation = Grid::new();
        for y in 0..GRID_ROWS {
            for x in 0..GRID_COLS {
                let alive_count = self.count_alive_neighbors(x, y);
                // next_gen cells are already all DEAD
                // next_generation.set_cell(x, y, DEAD);
                if self.get_cell(x, y) == ALIVE {
                    if alive_count == 2 || alive_count == 3 {
                        next_generation.set_cell(x, y, ALIVE);
                    }
                } else {
                    if alive_count == 3 {
                        next_generation.set_cell(x, y, ALIVE);
                    }
                }
            }
        }
        next_generation
    }

    fn print(&self) {
        print!("\x1b[H\x1b[2J\x1b[3J");
        for y in 0..GRID_ROWS {
            for x in 0..GRID_COLS {
                print!("{}", self.get_cell(x, y));
            }
            println!();
        }
        println!("Total population: {}", self.alive_count);
    }
}

fn main() {
    let mut grid = Grid::new();
    grid.fill_random();
    // grid.set_cell(10, 10, ALIVE);
    // grid.set_cell(11, 10, ALIVE);
    // grid.set_cell(12, 10, ALIVE);
    // grid.set_cell(12, 9, ALIVE);
    // grid.set_cell(11, 8, ALIVE);
    loop {
        grid.print();
        grid = grid.next_generation();
        sleep(Duration::from_millis(100));
    }
}

// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    
    [[false; COLS]; ROWS]
    
}

// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    
    for row in 0..ROWS {
    for col in 0..COLS {

        if grid[row][col] {
            print!("#");
        } else {
            print!(".");
        }
    }

    println!();

    }
}

// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;

    for r_offset in -1..=1 {
        for c_offset in -1..=1 {
     
            if r_offset == 0 && c_offset == 0 {
                continue;
            }

            let neighbor_row = row as isize + r_offset;
            let neighbor_col = col as isize + c_offset;

            if neighbor_row >= 0 && neighbor_row < 10 && neighbor_col >= 0 && neighbor_col < 20 {
                if grid[neighbor_row as usize][neighbor_col as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}

// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
     
        (true, 2..=3) => true,
        (true, _)     => false,
        
        (false, 3)    => true,
        (false, _)    => false,
    }
}

// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(_grid: &Grid) {
    todo!("Implement print_neighbor_counts")
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    todo!("Implement play")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    // TODO (student): write the remaining cases from your truth table before you
    // write next_state. At minimum: a live cell with 4 neighbors, a dead cell with 2.
    
    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn dead_2_remains_dead() {
        assert!(!next_state(false, 2));
    }

    #[test]
    fn live_0_dies() {
        assert!(!next_state(true, 0));
    }

    #[test]
    fn dead_8_remains_dead() {
        assert!(!next_state(false, 8));
    }
}


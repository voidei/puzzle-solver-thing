// prompt: how do I write a "lights out" puzzle solver that has a 3x3 grid using rust

const SIZE: usize = 3;

fn main() {
    let mut grid = [[false; SIZE]; SIZE]; // Initialize a 3x3 grid of lights

    // Example initial grid setup
    grid[0][0] = true;
    grid[0][1] = true;
    grid[1][0] = true;
    grid[1][1] = true;

    println!("Initial Grid:");
    print_grid(&grid);

    solve(&mut grid);

    println!("Solution:");
    print_grid(&grid);
}

fn solve(grid: &mut [[bool; SIZE]; SIZE]) {
    // Iterate through each cell in the grid
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Toggle the current cell and its adjacent cells
            toggle_cell(grid, i, j);
        }
    }
}

fn toggle_cell(grid: &mut [[bool; SIZE]; SIZE], i: usize, j: usize) {
    if i > 0 {
        grid[i - 1][j] = !grid[i - 1][j];
    }
    if i < SIZE - 1 {
        grid[i + 1][j] = !grid[i + 1][j];
    }
    if j > 0 {
        grid[i][j - 1] = !grid[i][j - 1];
    }
    if j < SIZE - 1 {
        grid[i][j + 1] = !grid[i][j + 1];
    }
    grid[i][j] = !grid[i][j]; // Toggle the current cell
}

fn print_grid(grid: &[[bool; SIZE]; SIZE]) {
    for row in grid {
        for &cell in row {
            if cell {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}

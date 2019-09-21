use reqwest;
use std::env;
use std::{fs, thread, time};
mod rle;

fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let body = if args.len() < 2 {
        fs::read_to_string("./rle_patterns/duodecapole.rle").expect("Could not read file.")
    } else {
        reqwest::get(&args[1])?.text()?
    };
    let rpe = rle::parse(body);
    let mut back_buffer = Vec::new();
    let mut front_buffer = Vec::new();
    copy_buffer(&rpe.grid, &mut back_buffer);
    copy_buffer(&rpe.grid, &mut front_buffer);
    let mut num_generations: u32 = 0;
    render(&front_buffer);
    loop {
        println!("{}", termion::clear::All);
        println!("Generations: {}", num_generations);
        process_generation(&mut back_buffer, &mut front_buffer, &rpe);
        render(&front_buffer);
        num_generations += 1;
        thread::sleep(time::Duration::from_millis(250)); // update every 1/4 second.
    }
}

/**
 * Copies a source 2D Vector into a target 2D Vector.
 */
fn copy_buffer(source: &Vec<Vec<u8>>, target: &mut Vec<Vec<u8>>) {
    for vec in source {
        let mut c = Vec::new();
        for e in vec {
            c.push(*e);
        }
        target.push(c);
    }
}

/**
 * Processes a generation based on the Game of Life rules defined
 * here: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
 */
fn process_generation(
    back_buffer: &mut Vec<Vec<u8>>,
    front_buffer: &mut Vec<Vec<u8>>,
    rpe: &rle::RLEParseEntity,
) -> () {
    // read from the back buffer and copy mutations into the
    // front buffer.
    let num_rows = rpe.rows;
    let num_columns = rpe.columns;
    for (i, row) in back_buffer.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut neighbors: u8 = 0;
            if i > 0 {
                neighbors += back_buffer[i - 1][j]; // up
                if j > 0 {
                    neighbors += back_buffer[i - 1][j - 1]; // up, left
                }
                if j < num_columns - 1 {
                    neighbors += back_buffer[i - 1][j + 1]; // up, right
                }
            }
            if i < num_rows - 1 {
                neighbors += back_buffer[i + 1][j]; // down
                if j > 0 {
                    neighbors += back_buffer[i + 1][j - 1]; // down, left
                }
                if j < num_columns - 1 {
                    neighbors += back_buffer[i + 1][j + 1]; // down, right
                }
            }

            if j > 0 {
                neighbors += back_buffer[i][j - 1]; // left
            }
            if j < num_columns - 1 {
                neighbors += back_buffer[i][j + 1]; // right
            }

            if *cell == rle::CELL_ALIVE {
                if neighbors < 2 {
                    front_buffer[i][j] = rle::CELL_DEAD;
                } else if neighbors < 4 {
                    front_buffer[i][j] = rle::CELL_ALIVE;
                } else {
                    front_buffer[i][j] = rle::CELL_DEAD;
                }
            } else {
                if neighbors == 3 {
                    front_buffer[i][j] = rle::CELL_ALIVE;
                }
            }
        }
    }
    // copy the mutated front buffer into the
    // back buffer.
    for (i, row) in front_buffer.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            back_buffer[i][j] = *column;
        }
    }
}

/**
 * Simple grid-rendering function that writes to stdout. Ultimately this should
 * be switched out for an SDL2 renderer -- this will allow the
 * program to render the pattern on an "infinite" grid instead
 * of being constrained to the terminal window. In addition, a new
 * renderer will allow us to visualize generations at a much faster
 * pace.
 * ie. I want to run this bad boi: http://www.conwaylife.com/w/index.php?title=Methuselah
 */
fn render(grid: &Vec<Vec<u8>>) -> () {
    print!("\n");
    for row in grid.iter() {
        print!("|");
        for column in row.iter() {
            if *column == rle::CELL_ALIVE {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        print!("|\n");
    }
}

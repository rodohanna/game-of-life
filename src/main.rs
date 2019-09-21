use reqwest;
use std::{thread, time};
mod rle;

fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("http://www.conwaylife.com/patterns/ak47reaction.rle")?.text()?;
    let rpe = rle::parse(body);
    let mut back_buffer = Vec::new();
    let mut front_buffer = Vec::new();
    copy_buffer(&rpe.grid, &mut back_buffer);
    copy_buffer(&rpe.grid, &mut front_buffer);
    let mut num_generations: u32 = 0;
    render(&front_buffer);
    thread::sleep(time::Duration::from_millis(3000));
    loop {
        println!("{}", termion::clear::All);
        println!("Num Generations: {}", num_generations);
        process_generation(&mut back_buffer, &mut front_buffer, &rpe);
        render(&front_buffer);
        num_generations += 1;
        thread::sleep(time::Duration::from_millis(100)); // update every 1/4 second.
    }
}

fn copy_buffer(source: &Vec<Vec<u8>>, target: &mut Vec<Vec<u8>>) {
    for vec in source {
        let mut c = Vec::new();
        for e in vec {
            c.push(*e);
        }
        target.push(c);
    }
}

fn process_generation(
    back_buffer: &mut Vec<Vec<u8>>,
    front_buffer: &mut Vec<Vec<u8>>,
    rpe: &rle::RLEParseEntity,
) -> () {
    // Copy back buffer into front buffer
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
    // Update back grid buffer
    for (i, row) in front_buffer.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            back_buffer[i][j] = *column;
        }
    }
}

fn render(grid: &Vec<Vec<u8>>) -> () {
    print!("\n\t\t\t");
    for row in grid.iter() {
        print!("|");
        for column in row.iter() {
            if *column == rle::CELL_ALIVE {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        print!("|\n\t\t\t");
    }
}

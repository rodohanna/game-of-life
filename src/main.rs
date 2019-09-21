use std::{thread, time};

mod rle;

const CELL_DEAD: u8 = 0;
const CELL_ALIVE: u8 = 1;

fn main() {
    let rle_entity = rle::parse_file(String::from("./rle_patterns/84p87.rle"));
    let mut back_grid_buffer = Vec::new();
    let mut front_grid_buffer = Vec::new();
    copy_vec(&rle_entity.grid, &mut back_grid_buffer);
    copy_vec(&rle_entity.grid, &mut front_grid_buffer);

    let mut num_generations: u32 = 0;
    loop {
        println!("{}", termion::clear::All);
        println!("Num Generations: {}", num_generations);
        process_generation(&mut back_grid_buffer, &mut front_grid_buffer, &rle_entity);
        print_grid(&front_grid_buffer);
        num_generations += 1;
        thread::sleep(time::Duration::from_millis(250)); // update every 1/4 second.
    }
}

fn copy_vec(source: &Vec<Vec<u8>>, target: &mut Vec<Vec<u8>>) {
    for vec in source {
        let mut c = Vec::new();
        for e in vec {
            c.push(*e);
        }
        target.push(c);
    }
}

fn process_generation(
    back_grid_buffer: &mut Vec<Vec<u8>>,
    front_grid_buffer: &mut Vec<Vec<u8>>,
    rle_parse_entity: &rle::RLEParseEntity,
) -> () {
    // Copy back buffer into front buffer
    let num_rows = rle_parse_entity
        .height
        .parse::<usize>()
        .expect("Could not convert HEIGHT to usize");;
    let num_columns = rle_parse_entity
        .width
        .parse::<usize>()
        .expect("Could not convert WIDTH to usize");
    for (i, row) in back_grid_buffer.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut neighbors: u8 = 0;
            if i > 0 {
                neighbors += back_grid_buffer[i - 1][j]; // up
                if j > 0 {
                    neighbors += back_grid_buffer[i - 1][j - 1]; // up, left
                }
                if j < num_columns - 1 {
                    neighbors += back_grid_buffer[i - 1][j + 1]; // up, right
                }
            }
            if i < num_rows - 1 {
                neighbors += back_grid_buffer[i + 1][j]; // down
                if j > 0 {
                    neighbors += back_grid_buffer[i + 1][j - 1]; // down, left
                }
                if j < num_columns - 1 {
                    neighbors += back_grid_buffer[i + 1][j + 1]; // down, right
                }
            }

            if j > 0 {
                neighbors += back_grid_buffer[i][j - 1]; // left
            }
            if j < num_columns - 1 {
                neighbors += back_grid_buffer[i][j + 1]; // right
            }

            if *cell == CELL_ALIVE {
                if neighbors < 2 {
                    front_grid_buffer[i][j] = CELL_DEAD;
                } else if neighbors < 4 {
                    front_grid_buffer[i][j] = CELL_ALIVE;
                } else {
                    front_grid_buffer[i][j] = CELL_DEAD;
                }
            } else {
                if neighbors == 3 {
                    front_grid_buffer[i][j] = CELL_ALIVE;
                }
            }
        }
    }
    // Update back grid buffer
    for (i, row) in front_grid_buffer.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            back_grid_buffer[i][j] = *column;
        }
    }
}

fn print_grid(grid: &Vec<Vec<u8>>) -> () {
    print!("\n\t\t\t");
    for row in grid.iter() {
        for column in row.iter() {
            if *column == CELL_ALIVE {
                print!("*");
            } else {
                print!(" ");
            }
        }
        print!("\n\t\t\t");
    }
}

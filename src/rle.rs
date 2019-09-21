use std::{cmp, fs, path::Path};

enum ParserState {
    BEGIN,
    COMMENT,
    WIDTH,
    HEIGHT,
    RULE,
    RunCount,
}

#[derive(Debug)]
pub struct RLEParseEntity {
    pub width: String,
    pub height: String,
    run_count: String,
    curr_row: usize,
    curr_col: usize,
    pub grid: Vec<Vec<u8>>,
}

pub fn parse_file(file_str: String) -> RLEParseEntity {
    let path = Path::new(&file_str);
    let blob = fs::read_to_string(path).expect("Could not read rle file.");
    parse(blob)
}

pub fn parse(blob: String) -> RLEParseEntity {
    let mut state = ParserState::BEGIN;
    let mut rle_entity = RLEParseEntity {
        width: String::from(""),
        height: String::from(""),
        run_count: String::from(""),
        curr_row: 0,
        curr_col: 0,
        grid: Vec::new(),
    };
    for c in blob.chars() {
        if c.is_whitespace() && c != '\n' {
            continue;
        }
        match state {
            ParserState::BEGIN => begin(&mut state, c),
            ParserState::COMMENT => comment(&mut state, c),
            ParserState::WIDTH => width(&mut state, c, &mut rle_entity),
            ParserState::HEIGHT => height(&mut state, c, &mut rle_entity),
            ParserState::RULE => rule(&mut state, c),
            ParserState::RunCount => run_count(&mut state, c, &mut rle_entity),
        }
    }
    rle_entity
}

fn begin(state: &mut ParserState, c: char) -> () {
    match c {
        '#' => *state = ParserState::COMMENT,
        'x' => *state = ParserState::WIDTH,
        _ => panic!("Unknown BEGIN char {}", c),
    }
}

fn comment(state: &mut ParserState, c: char) -> () {
    match c {
        '\n' => *state = ParserState::BEGIN,
        _ => {}
    }
}

fn width(state: &mut ParserState, c: char, rle_entity: &mut RLEParseEntity) -> () {
    match c {
        ',' => *state = ParserState::HEIGHT,
        '=' => {}
        _ => {
            if !c.is_numeric() {
                panic!("WIDTH char should be a number, not {}", c)
            }
            rle_entity.width.push(c)
        }
    }
}

fn height(state: &mut ParserState, c: char, rle_entity: &mut RLEParseEntity) -> () {
    match c {
        '\n' => *state = ParserState::BEGIN,
        ',' => *state = ParserState::RULE,
        '=' => {}
        'y' => {}
        _ => {
            if !c.is_numeric() {
                panic!("HEIGHT char should be a number, not {}", c)
            }
            rle_entity.height.push(c);

            let rows = rle_entity
                .height
                .parse::<i32>()
                .expect("Could not convert HEIGHT to int");
            let columns = rle_entity
                .width
                .parse::<i32>()
                .expect("Could not convert WIDTH to int");
            rle_entity.grid = Vec::new();
            for i in 0..rows {
                rle_entity.grid.push(Vec::with_capacity(columns as usize));
                for _ in 0..columns {
                    rle_entity.grid[i as usize].push(0)
                }
            }
        }
    }
}

fn rule(state: &mut ParserState, c: char) -> () {
    match c {
        '\n' => *state = ParserState::RunCount,
        _ => {}
    }
}

fn run_count(state: &mut ParserState, c: char, rle_entity: &mut RLEParseEntity) -> () {
    match c {
        '$' => {
            let width = rle_entity
                .width
                .parse::<usize>()
                .expect("Could not convert WIDTH to int");
            for i in rle_entity.curr_col..width {
                rle_entity.grid[rle_entity.curr_row][i] = 0;
            }
            let run_count = match rle_entity.run_count.parse::<usize>() {
                Ok(num) => num,
                _ => 1,
            };
            rle_entity.curr_row += run_count;
            rle_entity.curr_col = 0;
            rle_entity.run_count = String::from("");
            *state = ParserState::RunCount;
        }
        '!' => {}
        'b' => {
            let width = rle_entity
                .width
                .parse::<usize>()
                .expect("Could not convert WIDTH to int");
            let run_count = match rle_entity.run_count.parse::<usize>() {
                Ok(num) => num,
                _ => 1,
            };
            for i in 0..run_count {
                rle_entity.grid[rle_entity.curr_row][rle_entity.curr_col + i] = 0;
            }
            rle_entity.curr_col += cmp::min(run_count, width);
            rle_entity.run_count = String::from("");
        }
        'o' => {
            let width = rle_entity
                .width
                .parse::<usize>()
                .expect("Could not convert WIDTH to int");
            let run_count = match rle_entity.run_count.parse::<usize>() {
                Ok(num) => num,
                _ => 1,
            };
            for i in 0..run_count {
                rle_entity.grid[rle_entity.curr_row][rle_entity.curr_col + i] = 1;
            }
            rle_entity.curr_col += cmp::min(run_count, width - 1);
            rle_entity.run_count = String::from("");
        }
        '\n' => {}
        _ => {
            if !c.is_numeric() {
                panic!("RUN_COUNT char should be a number, not {}", c)
            }
            rle_entity.run_count.push(c)
        }
    }
}

use std::{cmp, fs, path::Path};

pub const CELL_DEAD: u8 = 0;
pub const CELL_ALIVE: u8 = 1;

enum ParserState {
    BEGIN,
    COMMENT,
    WIDTH,
    HEIGHT,
    RULE,
    TAG,
}

#[derive(Debug)]
pub struct RLEParseEntity {
    width: String,
    height: String,
    pub columns: usize,
    pub rows: usize,
    run_count: String,
    curr_row: usize,
    curr_col: usize,
    pub grid: Vec<Vec<u8>>,
}

#[allow(dead_code)]
pub fn parse_file(file_str: String) -> RLEParseEntity {
    let path = Path::new(&file_str);
    let blob = fs::read_to_string(path).expect("Could not read rle file.");
    parse(blob)
}

/**
 * Takes in a run-length encoded string and incrementally builds
 * an RLEParseEntity with the initial state of the pattern.
 * More info here:
 * https://en.wikipedia.org/wiki/Run-length_encoding
 * http://www.conwaylife.com/wiki/Run_Length_Encoded
 */
pub fn parse(blob: String) -> RLEParseEntity {
    let mut state = ParserState::BEGIN;
    let mut rpe = RLEParseEntity {
        width: String::from(""),
        height: String::from(""),
        columns: 0,
        rows: 0,
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
            ParserState::WIDTH => width(&mut state, c, &mut rpe),
            ParserState::HEIGHT => height(&mut state, c, &mut rpe),
            ParserState::RULE => rule(&mut state, c),
            ParserState::TAG => run_count(&mut state, c, &mut rpe),
        }
    }
    rpe
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

fn width(state: &mut ParserState, c: char, rpe: &mut RLEParseEntity) -> () {
    match c {
        ',' => {
            *state = ParserState::HEIGHT;
            let columns = rpe
                .width
                .parse::<usize>()
                .expect("Could not convert WIDTH to usize");
            rpe.columns = columns;
        }
        '=' => {}
        _ => {
            if !c.is_numeric() {
                panic!("WIDTH char should be numeric, not {}", c)
            }
            rpe.width.push(c)
        }
    }
}

fn height(state: &mut ParserState, c: char, rpe: &mut RLEParseEntity) -> () {
    match c {
        '\n' => *state = ParserState::BEGIN,
        ',' => *state = ParserState::RULE,
        '=' => {}
        'y' => {}
        _ => {
            if !c.is_numeric() {
                panic!("HEIGHT char should be a number, not {}", c)
            }
            rpe.height.push(c);
            let rows = rpe
                .height
                .parse::<usize>()
                .expect("Could not convert HEIGHT to usize");
            rpe.rows = rows;
            rpe.grid = Vec::new();
            for i in 0..rows {
                rpe.grid.push(Vec::with_capacity(rpe.columns));
                for _ in 0..rpe.columns {
                    rpe.grid[i].push(0)
                }
            }
        }
    }
}

fn rule(state: &mut ParserState, c: char) -> () {
    match c {
        '\n' => *state = ParserState::TAG,
        _ => {}
    }
}

fn run_count(state: &mut ParserState, c: char, rpe: &mut RLEParseEntity) -> () {
    match c {
        '$' => {
            for i in rpe.curr_col..rpe.columns {
                rpe.grid[rpe.curr_row][i] = 0;
            }
            let run_count = extract_run_count(rpe);
            rpe.curr_row += run_count;
            rpe.curr_col = 0;
            rpe.run_count = String::from("");
            *state = ParserState::TAG;
        }
        '!' => {}
        'b' => {
            let run_count = extract_run_count(rpe);
            for i in 0..run_count {
                rpe.grid[rpe.curr_row][rpe.curr_col + i] = CELL_DEAD;
            }
            rpe.curr_col += cmp::min(run_count, rpe.columns);
            rpe.run_count = String::from("");
        }
        'o' => {
            let run_count = extract_run_count(rpe);
            for i in 0..run_count {
                rpe.grid[rpe.curr_row][rpe.curr_col + i] = CELL_ALIVE;
            }
            rpe.curr_col += cmp::min(run_count, rpe.columns);
            rpe.run_count = String::from("");
        }
        '\n' => {}
        _ => {
            if !c.is_numeric() {
                panic!("RUN_COUNT char should be numeric, not {}", c)
            }
            rpe.run_count.push(c)
        }
    }
}

fn extract_run_count(rpe: &RLEParseEntity) -> usize {
    match rpe.run_count.parse::<usize>() {
        Ok(num) => num,
        _ => 1,
    }
}

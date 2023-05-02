use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Train {
    direction: Vec<i8>, // 0 = x, 1 = y
    future_direction: Vec<i8>,
    position: Vec<usize>,
    skip: bool,
}

#[derive(Clone)]
struct Memory {
    cells: Vec<u8>,
    position: usize,
}

fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn check_extention(filename: &str) {
    let splitted: Vec<&str> = filename.split(".").collect();
    if splitted[splitted.len() - 1] != "trainf" {
        panic!(
            "Expect a .trainf go a {} file",
            splitted[splitted.len() - 1]
        );
    }
}

fn find_in_row(
    y: usize,
    row: &str,
    symbol: &char,
    stations: &mut Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut x = 0;
    for chars in row.chars() {
        if &chars == symbol {
            stations.push(vec![x, y]);
        }
        x = x + 1;
    }
    stations.to_owned()
}

fn find_stations(code: &Vec<String>, symbol: char) -> Vec<Vec<usize>> {
    let mut stations: Vec<Vec<usize>> = Vec::new();
    let mut y = 0;
    for row in code {
        stations = find_in_row(y, row, &symbol, &mut stations);
        y = y + 1;
    }
    stations // 0 = x, 1 = y
}

fn remove_crashed(mut trains: Vec<Train>, position: &Vec<usize>) -> Vec<Train> {
    let mut i = 0;
    for train in trains.clone() {
        if train.position.eq(position) {
            trains.remove(i);
            continue;
        }
        i = i + 1;
    }
    trains
}

fn check_crashed(mut trains: Vec<Train>) -> (i32, Vec<Train>) {
    let mut positions = vec![];
    for train in &trains {
        positions.push(train.position.clone());
    }
    positions.sort();
    let mut i: usize = 1;
    for position in &positions {
        if i == positions.len() {
            break;
        }
        if position.eq(&positions[i]) {
            trains = remove_crashed(trains, &position);
        }
        i = i + 1;
    }
    (trains.len() as i32, trains)
}

fn turn_direction(mut direction: Vec<i8>) -> Vec<i8> {
    direction[0] = direction[0] * -1;
    direction[1] = direction[1] * -1;
    direction
}

fn move_after_turn(direction: &Vec<i8>, mut position: Vec<usize>) -> Vec<usize> {
    for _i in 0..2 {
        position = move_position(
                position.clone(),
                direction,
            );
    }
    position
}

fn move_position(mut position: Vec<usize>, direction: &Vec<i8>) -> Vec<usize> {
    for i in 0..direction.len() {
        position[i] = (position[i] as i32 + direction[i] as i32) as usize;
    }
    position
}

fn get_operator(code: &Vec<String>, position: &Vec<usize>) -> char {
    if position[1] >= code.len() || position[0] >= code[position[1]].len() {
        ' '
    } else {
        code[position[1]].chars().nth(position[0]).unwrap()
    }
}

fn change_value(value: u8, change: i8) -> u8 {
    match value as i16 - change as i16 {
        -1 => 255,
        256 => 0,
        _ => (value as i8 - change) as u8,
    }
}

fn change_cell(mut memory: Memory, direction_0: usize) -> Memory {
    let new_cell_position = (memory.position + direction_0) as i32;
    if new_cell_position < 0 {
        memory.cells = [vec![0], memory.cells].concat();
        memory.position = 0;
        return memory;
    }
    memory.position = new_cell_position as usize;
    if memory.position + 1 - memory.cells.len() > 0 {
        memory.cells.push(0);
    }
    memory
}

fn operators(train: &Train, mut operator: char, mut memory: Memory, code: Vec<String>) -> (Train, Memory) {
    let mut this_train = train.clone();
    if operator.to_owned() == '|' {
        if this_train.direction[1] != 0 {
            return (this_train, memory);
        }
        this_train.direction = turn_direction(this_train.direction.clone());
        this_train.position = move_after_turn(&this_train.direction, this_train.position.clone());    
        operator = get_operator(&code, &this_train.position);         
        } else if operator.to_owned() == '-' {
            if this_train.direction[0] != 0 {
                return (this_train, memory);
        }
        this_train.direction = turn_direction(this_train.direction.clone());
        this_train.position = move_after_turn(&this_train.direction, this_train.position.clone());    
        operator = get_operator(&code, &this_train.position);         
    } else if operator.to_owned() == ' ' {
        this_train.direction = turn_direction(this_train.direction.clone());
        this_train.position = move_after_turn(&this_train.direction, this_train.position.clone());    
        operator = get_operator(&code, &this_train.position);         
    }
    if this_train.skip == true {
        this_train.skip = false;
        return (this_train, memory);
    }
    match operator {
        '^' => this_train.future_direction = vec![0, -1],
        'v' => this_train.future_direction = vec![0, 1],
        '>' => this_train.future_direction = vec![1, 0],
        '<' => this_train.future_direction = vec![-1, 0],
        'o' => this_train.direction = this_train.future_direction.clone(),
        '+' => {
            if this_train.direction[0] != 0 {
                memory = change_cell(memory.clone(), this_train.direction[0] as usize);
            } else {
                memory.cells[memory.position] =
                    change_value(memory.cells[memory.position], this_train.direction[1]);
            }
        }
        '.' => print!("{}", memory.cells[memory.position] as char),
        ',' => {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read input");
            memory.cells[memory.position] = input.bytes().nth(0).expect("failed to read byte");
        }
        '?' => {
            if memory.position != 0
                && memory.cells[memory.position] == memory.cells[memory.position - 1]
                || memory.position == 0 && memory.cells[255] == memory.cells[0]
            {
                this_train.skip = true;
            }
        }
        '|' | '-' => { return (this_train, memory); }  
        _ => panic!(
            "Illegal operator \"{}\" at: {:?} ",
            operator, this_train.position
        ),
    }
    (this_train, memory)
}

fn rand_train(trains_num: i32) -> Vec<i32> {
    let mut trains_order: Vec<i32> = (0..trains_num).map(|v| v).collect();
    let mut rng = thread_rng();
    trains_order.shuffle(&mut rng);
    trains_order
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 file, got {}", (args.len() - 1).to_string())
    }
    check_extention(&args[1]);
    let code = read_file(&args[1])?;
    let stations = find_stations(&code, '+');
    let mut memory = Memory {
        cells: vec![0],
        position: 0,
    };
    let mut trains: Vec<Train> = Vec::new();
    let dircs: Vec<Vec<i8>> = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
    for curr_station in &stations {
        let mut dirc_var = 0;
        while dirc_var < 4 {
            trains.push(Train {
                direction: dircs[dirc_var].clone(),
                future_direction: dircs[dirc_var].clone(),
                position: curr_station.clone(),
                skip: false,
            });
            dirc_var = dirc_var + 1;
        }
    }
    let mut trains_num = stations.len() as i32 * 4;
    while trains_num > 0 {
        let trains_order = rand_train(trains_num);
        for i in trains_order {
            trains[i as usize].position = move_position(
                trains[i as usize].position.clone(),
                &trains[i as usize].direction,
            );
            let operator = get_operator(&code, &trains[i as usize].position.clone());
            (trains[i as usize], memory) =
                operators(&trains[i as usize], operator, memory.clone(), code.clone());
        }
        (trains_num, trains) = check_crashed(trains);
    }
    Ok(())
}

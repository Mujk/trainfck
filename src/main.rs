use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Train {
    dirc: Vec<i8>,
    futr_dirc: Vec<i8>,
    pos: Vec<usize>,
    ignore: bool,
}

fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn check_ext(filename: &str) {
    let splitted: Vec<&str> = filename.split(".").collect();
    if splitted[splitted.len() - 1] == ".trainf" {
        panic!(
            "Expect a .trainf go a {} file",
            splitted[splitted.len() - 1]
        );
    }
}

fn find_stations(code: &Vec<String>, symbol: char) -> Vec<Vec<usize>> {
    let mut stations: Vec<Vec<usize>> = Vec::new();
    let mut row = 0;
    for rows in code {
        stations.push(
            rows.chars()
                .enumerate()
                .filter(|(_, c)| *c == symbol)
                .map(|(i, _)| i)
                .collect::<Vec<_>>(),
        );
        if !stations[row].is_empty() {
            stations[row].push(row);
        }
        row = row + 1;
    }
    stations.retain(|x| !x.is_empty());
    stations // 0 = x, 1 = y
}

fn remove_crashed(mut trains: Vec<Train>, pos: &Vec<usize>) -> Vec<Train> {
    let mut i = 0;
    for train in trains.clone() {
        if train.pos.eq(pos) {
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
        positions.push(train.pos.clone());
    }
    positions.sort();
    let mut i: usize = 1;
    for pos in &positions {
        if i == positions.len() {
            break;
        }
        if pos.eq(&positions[i]) {
            trains = remove_crashed(trains, &pos);
        }
        i = i + 1;
    }
    (trains.len() as i32, trains)
}

fn turn_dirc(mut dirc: Vec<i8>) -> Vec<i8> {
    dirc[0] = dirc[0] * -1;
    dirc[1] = dirc[1] * -1;
    dirc
}

fn move_pos(mut pos: Vec<usize>, dirc: &Vec<i8>) -> Vec<usize> {
    for i in 0..dirc.len() {
        pos[i] = (pos[i] as i32 + dirc[i] as i32) as usize;
    }
    pos
}

fn get_opr(code: &Vec<String>, pos: &Vec<usize>) -> char {
    if pos[1] >= code.len() || pos[0] >= code[pos[1]].len() {
        ' '
    } else {
        code[pos[1]].chars().nth(pos[0]).unwrap()
    }
}

fn change_val(val: u8, change: i8) -> u8 {
    match val as i16 - change as i16 {
        -1 => 255,
        256 => 0,
        _ => (val as i8 - change) as u8,
    }
}

fn change_cell(mut cell_pos: i32, mut cells: Vec<u8>, dirc_0: usize) -> (Vec<u8>, usize) {
    cell_pos = cell_pos + dirc_0 as i32;
    if cell_pos < 0 {
        cells = [vec![0], cells].concat();
        cell_pos = 0;
    } else if cell_pos + 1 - cells.len() as i32 > 0 {
        cells.push(0);
    }
    (cells, cell_pos as usize)
}

fn operators(
    train: &Train,
    opr: &char,
    mut cell_pos: usize,
    mut cells: Vec<u8>,
) -> (Train, Vec<u8>, usize) {
    let mut this_train = train.clone();
    if opr.to_owned() == '|' {
        if this_train.dirc[1] == 0 {
            this_train.dirc = turn_dirc(this_train.dirc);
        }
        return (this_train, cells, cell_pos);
    } else if opr.to_owned() == '-' {
        if this_train.dirc[0] == 0 {
            this_train.dirc = turn_dirc(this_train.dirc);
        }
        return (this_train, cells, cell_pos);
    } else if opr.to_owned() == ' ' {
        this_train.dirc = turn_dirc(this_train.dirc);
        return (this_train, cells, cell_pos);
    }
    if this_train.ignore == true {
        this_train.ignore = false;
        return (this_train, cells, cell_pos);
    }
    match opr {
        '^' => this_train.futr_dirc = vec![0, -1],
        'v' => this_train.futr_dirc = vec![0, 1],
        '>' => this_train.futr_dirc = vec![1, 0],
        '<' => this_train.futr_dirc = vec![-1, 0],
        'o' => this_train.dirc = this_train.futr_dirc.clone(),
        '+' => {
            if this_train.dirc[0] != 0 {
                (cells, cell_pos) =
                    change_cell(cell_pos as i32, cells, this_train.dirc[0] as usize);
            } else {
                cells[cell_pos] = change_val(cells[cell_pos], this_train.dirc[1]);
            }
        }
        '.' => println!("{}", cells[cell_pos] as char),
        ',' => {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read input");
            cells[cell_pos] = input.bytes().nth(0).expect("failed to read byte");
        }
        '?' => {
            if cell_pos != 0 && cells[cell_pos] == cells[cell_pos - 1] {
                this_train.ignore = true;
            }
        }
        _ => panic!("Illegal operator \"{}\" at: {:?} ", opr, this_train.pos),
    }
    (this_train, cells, cell_pos)
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
    check_ext(&args[1]);
    let code = read_file(&args[1])?;
    let stations = find_stations(&code, '+');
    let mut trains: Vec<Train> = Vec::new();
    let dircs: Vec<Vec<i8>> = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
    for curr_station in &stations {
        let mut dirc_var = 0;
        while dirc_var < 4 {
            trains.push(Train {
                dirc: dircs[dirc_var].clone(),
                futr_dirc: dircs[dirc_var].clone(),
                pos: curr_station.clone(),
                ignore: false,
            });
            dirc_var = dirc_var + 1;
        }
    }
    let mut cell_pos: usize = 0;
    let mut cells: Vec<u8> = vec![0];
    let mut trains_num = stations.len() as i32 * 4;
    while trains_num > 0 {
        let trains_order = rand_train(trains_num);
        for i in trains_order {
            trains[i as usize].pos =
                move_pos(trains[i as usize].pos.clone(), &trains[i as usize].dirc);
            let opr = get_opr(&code, &trains[i as usize].pos.clone());
            (trains[i as usize], cells, cell_pos) =
                operators(&trains[i as usize], &opr, cell_pos, cells.clone());
        }
        (trains_num, trains) = check_crashed(trains);
    }
    Ok(())
}

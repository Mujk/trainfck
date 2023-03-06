use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

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

fn check_crashed(mut trains: Vec<Train>) -> (i32, Vec<Train>) {
    let mut positions = vec![];
    for train in &trains {
        positions.push(train.pos.clone());
    }
    positions.sort();
    let all_pos: Vec<Vec<usize>> = positions.clone();
    positions.dedup();
    println!("{:?}", positions);
    let mut i = 1;
    let mut crashed = vec![];
    for pos in all_pos {
        if i < positions.len() && pos.eq(&positions[i]) {
            println!("crash!, {:?}", pos);
            crashed.push(pos);
            continue;
        }
        i = i + 1;
    }
    crashed.sort();
    crashed.dedup();
    let loop_trains = trains.clone();
    for train in loop_trains {
        i = 0;
        for crash in &crashed {
            if &train.pos == crash {
                trains.remove(i);
            }
            i = i + 1;
        }
    }
    (trains.len() as i32, trains)
}

fn turn_dirc(mut dirc: Vec<i8>) -> Vec<i8> {
    for i in 0..dirc.len() - 1 {
        dirc[i] = dirc[i] * -1;
    }
    dirc
}

fn move_pos(mut pos: Vec<usize>, dirc: &Vec<i8>) -> Vec<usize> {
    for i in 0..dirc.len() {
        pos[i] = (pos[i] as i32 + dirc[i] as i32) as usize;
    }
    pos
}

fn get_opr(code: &Vec<String>, pos: &Vec<usize>) -> char {
    code[pos[1]].chars().nth(pos[0]).unwrap()
}

fn change_cell(mut cell_pos: i16, mut cells: Vec<u8>) -> (usize, Vec<u8>) {
    if cell_pos == 0 {
        cell_pos = 255;
    } else if cell_pos == 255 {
        cell_pos = 0;
    }
    while cell_pos as i16 - cells.len() as i16 > 0 {
        cells.push(0);
    }
    (cell_pos as usize, cells)
}

fn operators(
    train: &Train,
    opr: &char,
    mut cell_pos: usize,
    mut cells: Vec<u8>,
) -> (Train, Vec<u8>, usize) {
    let mut this_train = train.clone();
    if opr.to_owned() == '|' || opr.to_owned() == '-' {
        return (this_train, cells, cell_pos);
    }
    if this_train.ignore == true {
        this_train.ignore = false;
        return (this_train, cells, cell_pos);
    }
    println!("{:?}: {}", train.pos, opr);
    match opr {
        '^' => this_train.futr_dirc = vec![0, 1],
        'v' => this_train.futr_dirc = vec![0, -1],
        '>' => this_train.futr_dirc = vec![1, 0],
        '<' => this_train.futr_dirc = vec![-1, 0],
        'o' => this_train.dirc = this_train.futr_dirc.clone(),
        '+' => {
            if this_train.dirc[0] != 0 {
                (cell_pos, cells) = change_cell(cell_pos as i16 + this_train.dirc[0] as i16, cells);
            } else {
                cells[cell_pos] = cells[cell_pos] + this_train.dirc[1] as u8;
            }
        }
        '.' => println!("{}", cells[cell_pos] as char),
        ',' => {
            cells[cell_pos] = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u8)
                .unwrap()
        }
        '!' => {
            if cells[cell_pos] == 0 {
                this_train.ignore = true;
            }
        }
        _ => this_train.dirc = turn_dirc(this_train.dirc),
    }
    (this_train, cells, cell_pos)
}

fn main() -> io::Result<()> {
    #[allow(unused)]
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
        println!("{}", trains_num);
        for i in 0..trains_num {
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

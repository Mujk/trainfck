use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Train {
    dirc: Vec<i8>,
    futr_dirc: Vec<i8>,
    pos: Vec<usize>,
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

fn find_stations(code: Vec<String>, symbol: char) -> Vec<Vec<usize>> {
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

fn check_crashed(trains: &mut Vec<Train>) -> &Vec<Train> {
    let mut positions = vec![];
    for train in &mut *trains {
        positions.push(train.pos.clone());
    }
    positions.sort();
    let all_pos: Vec<Vec<usize>> = positions.clone();
    positions.dedup();
    let mut i = 0;
    let mut crashed = vec![];
    for pos in all_pos {
        if i < positions.len() && pos.eq(&positions[i]) {
            crashed.push(pos);
            continue;
        }
        i = i + 1;
    }
    let crash_num = crashed.len() as i32;
    crashed.sort();
    crashed.dedup();
    i = 0;
    let loop_trains = trains.clone();
    for train in loop_trains {
        for crash in &crashed {
            if &train.pos == crash {
                trains.remove(i);
            }
        }
    }
    trains
}

fn operators(train: &mut Train, opr: char) -> &Train {
    match opr {
        '^' => train.futr_dirc[1] = 1,
    }
}

fn main() -> io::Result<()> {
    #[allow(unused)]
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 file, got {}", (args.len() - 1).to_string())
    }
    check_ext(&args[1]);
    let code = read_file(&args[1])?;
    let stations = find_stations(code, '+');
    let mut trains: Vec<Train> = Vec::new();
    let mut trains_num = stations.len() as i32;
    let dircs: Vec<Vec<i8>> = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
    for curr_station in &stations {
        let mut dirc_var = 0;
        while dirc_var < 4 {
            trains.push(Train {
                dirc: dircs[dirc_var].clone(),
                futr_dirc: dircs[dirc_var].clone(),
                pos: curr_station.clone(),
            });
            dirc_var = dirc_var + 1;
        }
    }
    let cells: Vec<i8> = Vec::new();
    while trains_num > 0 {
        let trains = check_crashed(&mut trains);
    }
    Ok(())
}

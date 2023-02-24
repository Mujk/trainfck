use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Train {
    dirc: [i8; 2],
    futr_dirc: [i8; 2],
    pos: [i32; 2],
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
        let mut stat: Vec<i32>;
        row = row + 1;
    }
    stations.retain(|x| !x.is_empty());
    stations // 0 = x, 1 = y
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 file, got {}", (args.len() - 1).to_string())
    }
    check_ext(&args[1]);
    let code = read_file(&args[1])?;
    let stations = find_stations(code, '+');
    let mut trains: Vec<Train> = Vec::new();
    let trains_num = stations.len();
    let dircs: Vec<Vec<i8>> = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
    for curr_station in stations {
        let mut dirc_var = 0;
        while dirc_var < 4 {
            trains.push(Train {
                dirc: dircs[dirc_var].try_into().unwrap(),
                futr_dirc: dircs[dirc_var].try_into().unwrap(),
                pos: curr_station.try_into().unwrap(),
            });
            dirc_var = dirc_var + 1;
        }
    }
    //let cells: Vec<i8> = Vec::new();
    let mut not_crashed = true;
    while not_crashed {
        println!("Hi");
        not_crashed = false;
    }
    Ok(())
}

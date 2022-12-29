use std::{fs::File, io::{BufReader, BufRead, Read}};

#[cfg(test)]
mod day01_tests;
mod day02_tests;
mod day03_tests;
mod day04_tests;
mod day05_tests;
mod day06_tests;
mod day07_tests;
mod day08_tests;
mod day09_tests;
mod day10_tests;
mod day11_tests;

pub fn read_file(path: &str) -> Vec<String> {
    match File::open(path) {
        Ok(f) => BufReader::new(f).lines().map(|it| it.unwrap()).collect(),
        Err(_) => panic!("Could not oepn file"),
    }
}

pub fn read_file_as_string(path: &str) -> String {
    match File::open(path) {
        Ok(f) => {
            let mut file_data = String::new();
            match BufReader::new(f).read_to_string(&mut file_data) {
                Ok(_) => (),
                Err(_) => panic!("Could not read file: '{}'", path),
            }
            file_data
        }
        Err(_) => panic!("Could not oepn file: '{}'", path),
    }
}

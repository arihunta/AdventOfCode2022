use std::{fs::File, io::{BufReader, BufRead}};

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

pub fn read_file(path: &str) -> Vec<String> {
    match File::open(path) {
        Ok(f) => BufReader::new(f).lines().map(|it| it.unwrap()).collect(),
        Err(_) => panic!("Could not oepn file"),
    }
}

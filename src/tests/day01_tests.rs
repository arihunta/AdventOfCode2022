
use std::{fs::File, io::{BufReader, BufRead}};
use crate::day01;

#[test]
fn test_day01_01_01() {

    let f = match File::open("data/01_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day01::day01_01(BufReader::new(f).lines());

    assert_eq!(24_000, result);

}

#[test]
fn test_day01_01() {

    let f = match File::open("data/01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day01::day01_01(BufReader::new(f).lines());

    assert_eq!(70_369, result);

}

#[test]
fn test_day01_02_01() {

    let f = match File::open("data/01_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day01::day01_02(BufReader::new(f).lines());

    assert_eq!(45_000, result);

}

#[test]
fn test_day01_02() {

    let f = match File::open("data/01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day01::day01_02(BufReader::new(f).lines());

    assert_eq!(203_002, result);

}

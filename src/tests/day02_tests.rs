
use std::{fs::File, io::{BufReader, BufRead}};
use crate::day02;

#[test]
fn test_day02_01_01() {

    let f = match File::open("data/02_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day02::day02_01(BufReader::new(f).lines());

    assert_eq!(15, result);

}

#[test]
fn test_day02_01() {

    let f = match File::open("data/02") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day02::day02_01(BufReader::new(f).lines());

    assert_eq!(12_535, result);

}

#[test]
fn test_day02_02_01() {

    let f = match File::open("data/02_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day02::day02_02(BufReader::new(f).lines());

    assert_eq!(12, result);

}

#[test]
fn test_day02_02() {

    let f = match File::open("data/02") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day02::day02_02(BufReader::new(f).lines());

    assert_eq!(15_457, result);

}

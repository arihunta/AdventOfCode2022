
use std::{fs::File, io::{BufReader, BufRead}};
use crate::day04;

#[test]
fn test_day04_01_01() {

    let f = match File::open("data/04_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day04::day04_01(BufReader::new(f).lines());

    assert_eq!(2, result);

}

#[test]
fn test_day04_01() {

    let f = match File::open("data/04") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day04::day04_01(BufReader::new(f).lines());

    assert_eq!(515, result);

}

#[test]
fn test_day04_02_01() {

    let f = match File::open("data/04_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day04::day04_02(BufReader::new(f).lines());

    assert_eq!(4, result);

}

#[test]
fn test_day04_02() {

    let f = match File::open("data/04") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day04::day04_02(BufReader::new(f).lines());

    assert_eq!(883, result);

}

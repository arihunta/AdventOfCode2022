
use std::{fs::File, io::{BufReader, BufRead}};
use crate::day03;

#[test]
fn test_day03_01_01() {

    let f = match File::open("data/03_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day03::day03_01(BufReader::new(f).lines());

    assert_eq!(157, result);

}

#[test]
fn test_day03_01() {

    let f = match File::open("data/03") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day03::day03_01(BufReader::new(f).lines());

    assert_eq!(7_716, result);

}

#[test]
fn test_day03_02_01() {

    let f = match File::open("data/03_01") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day03::day03_02(BufReader::new(f).lines());

    assert_eq!(70, result);

}

#[test]
fn test_day03_02() {

    let f = match File::open("data/03") {
        Ok(f) => f,
        Err(_) => panic!("Could not oepn file"),
    };

    let result = day03::day03_02(BufReader::new(f).lines());

    assert_eq!(2_973, result);

}

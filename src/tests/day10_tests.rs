
use crate::{day10, tests::read_file};

#[test]
fn test_day10_01_01() {

    let input = read_file("data/10_01");

    let result = day10::day10_01(input);

    assert_eq!(13140, result);

}

#[test]
fn test_day10_01() {

    let input = read_file("data/10");

    let result = day10::day10_01(input);

    assert_eq!(14240, result);

}

#[test]
fn test_day10_02_01() {

    let input = read_file("data/10_01");

    let result = day10::day10_02(input);

    assert_eq!("PLULKBZH", result);

}

#[test]
fn test_day10_02() {

    let input = read_file("data/10");

    let result = day10::day10_02(input);

    assert_eq!("PLULKBZH", result);

}

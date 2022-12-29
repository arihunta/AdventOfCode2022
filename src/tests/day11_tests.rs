
use crate::{day11, tests::{read_file_as_string}};

#[test]
fn test_day11_01_01() {

    let input = read_file_as_string("data/11_01");

    let result = day11::day11_01(input);

    assert_eq!(10605, result);

}

#[test]
fn test_day11_01() {

    let input = read_file_as_string("data/11");

    let result = day11::day11_01(input);

    assert_eq!(66802, result);

}

#[test]
fn test_day11_02_01() {

    let input = read_file_as_string("data/11_01");

    let result = day11::day11_02(input);

    assert_eq!(2713310158, result);

}

#[test]
fn test_day11_02() {

    let input = read_file_as_string("data/11");

    let result = day11::day11_02(input);

    assert_eq!(21800916620, result);

}

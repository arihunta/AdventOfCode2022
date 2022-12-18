
use crate::{day07, tests::read_file};

#[test]
fn test_day07_01_01() {

    let input = read_file("data/07_01");

    let result = day07::day07_01(input);

    assert_eq!(95437, result);

}

#[test]
fn test_day07_01() {

    let input = read_file("data/07");

    let result = day07::day07_01(input);

    assert_eq!(1792222, result);

}

#[test]
fn test_day07_02_01() {

    let input = read_file("data/07_01");

    let result = day07::day07_02(input);

    assert_eq!(24933642, result);

}

#[test]
fn test_day07_02() {

    let input = read_file("data/07");

    let result = day07::day07_02(input);

    assert_eq!(1112963, result);

}

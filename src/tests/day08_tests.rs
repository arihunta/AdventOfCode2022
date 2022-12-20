
use crate::{day08, tests::read_file};

#[test]
fn test_day08_01_01() {

    let input = read_file("data/08_01");

    let result = day08::day08_01(input);

    assert_eq!(21, result);

}

#[test]
fn test_day08_01() {

    let input = read_file("data/08");

    let result = day08::day08_01(input);

    assert_eq!(1711, result);

}

#[test]
fn test_day08_02_01() {

    let input = read_file("data/08_01");

    let result = day08::day08_02(input);

    assert_eq!(8, result);

}

#[test]
fn test_day08_02() {

    let input = read_file("data/08");

    let result = day08::day08_02(input);

    assert_eq!(301392, result);

}

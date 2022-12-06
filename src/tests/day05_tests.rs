
use crate::{day05, tests::read_file};

#[test]
fn test_day05_01_01() {

    let input = read_file("data/05_01");

    let result = day05::day05_01(input);

    assert_eq!("CMZ", result);

}

#[test]
fn test_day05_01() {

    let input = read_file("data/05");

    let result = day05::day05_01(input);

    assert_eq!("SVFDLGLWV", result);

}

#[test]
fn test_day05_02_01() {

    let input = read_file("data/05_01");

    let result = day05::day05_02(input);

    assert_eq!("MCD", result);

}

#[test]
fn test_day05_02() {

    let input = read_file("data/05");

    let result = day05::day05_02(input);

    assert_eq!("DCVTCVPCL", result);

}

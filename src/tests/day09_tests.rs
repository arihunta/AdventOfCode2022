
use crate::{day09, tests::read_file};

#[test]
fn test_day09_01_01() {

    let input = read_file("data/09_01");

    let result = day09::day09_01(input);

    assert_eq!(13, result);

}

#[test]
fn test_day09_01() {

    let input = read_file("data/09");

    let result = day09::day09_01(input);

    assert_eq!(6384, result);

}

#[test]
fn test_day09_02_01() {

    let input = read_file("data/09_01");

    let result = day09::day09_02(input);

    assert_eq!(1, result);

}

#[test]
fn test_day09_02_02() {

    let input = read_file("data/09_02");

    let result = day09::day09_02(input);

    assert_eq!(36, result);

}

#[test]
fn test_day09_02() {

    let input = read_file("data/09");

    let result = day09::day09_02(input);

    assert_eq!(301392, result);

}

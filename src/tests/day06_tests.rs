
use crate::{day06, tests::read_file};

#[test]
fn test_day06_01_01() {

    let mut input = read_file("data/06_01");

    let result = day06::day06_01(input.remove(0));

    assert_eq!(7, result);

}

#[test]
fn test_day06_01_02() {

    let mut input = read_file("data/06_02");

    let result = day06::day06_01(input.remove(0));

    assert_eq!(5, result);

}

#[test]
fn test_day06_01_03() {

    let mut input = read_file("data/06_03");

    let result = day06::day06_01(input.remove(0));

    assert_eq!(6, result);

}

#[test]
fn test_day06_01_04() {

    let mut input = read_file("data/06_04");

    let result = day06::day06_01(input.remove(0));

    assert_eq!(10, result);

}

#[test]
fn test_day06_01_05() {

    let mut input = read_file("data/06_05");

    let result = day06::day06_01(input.remove(0));

    assert_eq!(11, result);

}

#[test]
fn test_day06_01() {

    let mut input = read_file("data/06");

    let result = day06::day06_01(input.remove(0));

    assert_eq!(1953, result);

}

#[test]
fn test_day06_02_01() {

    let mut input = read_file("data/06_01");

    let result = day06::day06_02(input.remove(0));

    assert_eq!(19, result);

}

#[test]
fn test_day06_02_02() {

    let mut input = read_file("data/06_02");

    let result = day06::day06_02(input.remove(0));

    assert_eq!(23, result);

}

#[test]
fn test_day06_02_03() {

    let mut input = read_file("data/06_03");

    let result = day06::day06_02(input.remove(0));

    assert_eq!(23, result);

}

#[test]
fn test_day06_02_04() {

    let mut input = read_file("data/06_04");

    let result = day06::day06_02(input.remove(0));

    assert_eq!(29, result);

}

#[test]
fn test_day06_02_05() {

    let mut input = read_file("data/06_05");

    let result = day06::day06_02(input.remove(0));

    assert_eq!(26, result);

}

#[test]
fn test_day06_02() {

    let mut input = read_file("data/06");

    let result = day06::day06_02(input.remove(0));

    assert_eq!(2301, result);

}
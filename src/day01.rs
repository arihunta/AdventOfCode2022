use std::{io::{Lines, BufReader}, fs::File};

pub fn day01_01(lines: Lines<BufReader<File>>) -> i32 {

    let mut max_cals : i32 = 0;
    let mut current_sum : i32 = 0;

    for x in lines {
        let line = x.unwrap();
        if line == "" {
            max_cals = std::cmp::max(max_cals, current_sum);
            current_sum = 0;
        }
        else {
            current_sum += line.parse::<i32>().unwrap()
        }
    }

    return std::cmp::max(max_cals, current_sum);

}

pub fn day01_02(lines: Lines<BufReader<File>>) -> i32 {

    let mut max_cals : [i32; 3] = [0, 0, 0];
    let mut current_sum : i32 = 0;

    for x in lines {
        let line = x.unwrap();
        if line == "" {
            max_cals[0] = std::cmp::max(max_cals[0], current_sum);
            max_cals.sort();
            current_sum = 0;
        }
        else {
            current_sum += line.parse::<i32>().unwrap()
        }
    }
    max_cals[0] = std::cmp::max(max_cals[0], current_sum);
    max_cals.sort();

    return max_cals[0] + max_cals[1] + max_cals[2];

}

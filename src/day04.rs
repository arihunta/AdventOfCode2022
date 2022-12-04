use std::{io::{Lines, BufReader}, fs::File};

pub fn day04_01(lines: Lines<BufReader<File>>) -> i32 {

    return lines
        .map( |line| line.unwrap() )
        .map( |line| line.split(&[',','-'])
                .map( |num| num.parse::<i32>().unwrap() )
                .collect::<Vec<i32>>() 
            )
        .filter( |sections| (sections[0] <= sections[2] && sections[1] >= sections[3]) 
                    || (sections[2] <= sections[0] && sections[3] >= sections[1]) )
        .count().try_into().unwrap()
    ;

}

pub fn day04_02(lines: Lines<BufReader<File>>) -> i32 {

    return lines
        .map( |line| line.unwrap() )
        .map( |line| line.split(&[',','-'])
                .map( |num| num.parse::<i32>().unwrap() )
                .collect::<Vec<i32>>() 
            )
        .filter( |sections| (sections[2] >= sections[0] && sections[2] <= sections[1])
                    || (sections[3] >= sections[0] && sections[3] <= sections[1])
                    || (sections[0] >= sections[2] && sections[0] <= sections[3])
                    || (sections[1] >= sections[2] && sections[1] <= sections[3]) )
        .count().try_into().unwrap()
    ;

}

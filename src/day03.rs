use std::{io::{Lines, BufReader}, fs::File, collections::HashSet};

pub fn day03_01(lines: Lines<BufReader<File>>) -> i32 {

    let mut priority_sum : i32 = 0;

    for rucksack in lines {
        let rucksack = rucksack.unwrap();
        let compartment1 = &rucksack[..(rucksack.len() / 2)];
        let compartment2 = &rucksack[(rucksack.len() / 2)..];

        let mut x = HashSet::new();

        for chr in compartment1.bytes() {
            x.insert(chr);
        }

        for chr in compartment2.bytes() {
            if x.contains(&chr) {
                priority_sum += to_priority(chr);
                break;
            }
        }

    }

    return priority_sum;

}

pub fn day03_02(lines: Lines<BufReader<File>>) -> i32 {

    let mut priority_sum : i32 = 0;

    // for rucksack in lines {
    //     let rucksack = rucksack.unwrap();
    //     let compartment1 = &rucksack[..(rucksack.len() / 2)];
    //     let compartment2 = &rucksack[(rucksack.len() / 2)..];

    //     let mut x = HashSet::new();

    //     x.

    //     for chr in compartment1.bytes() {
    //         x.insert(chr);
    //     }

    //     for chr in compartment2.bytes() {
    //         if x.contains(&chr) {
    //             priority_sum += to_priority(chr);
    //             break;
    //         }
    //     }

    // }

    return priority_sum;

}

fn to_priority(item: u8) -> i32 {
    if item < 91 {
        i32::from(item) - 38
    } else {
        i32::from(item) - 96
    }
}

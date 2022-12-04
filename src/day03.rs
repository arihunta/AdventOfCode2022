use std::{io::{Lines, BufReader}, fs::File, collections::HashSet};

pub fn day03_01(lines: Lines<BufReader<File>>) -> i32 {

    return lines
        .map( |line| line.unwrap() )
        .map( |rucksack| -> i32 {
            let compartment1 = &rucksack[..(rucksack.len() / 2)];
            let compartment2 = &rucksack[(rucksack.len() / 2)..];

            let compartment1 : HashSet<char> = HashSet::from_iter(compartment1.chars());

            match compartment2.chars().find( |chr| compartment1.contains(chr) ) {
                Some(chr) => to_priority(chr as i32),
                None => 0,
            }
        } )
        .sum();

}

pub fn day03_02(mut lines: Lines<BufReader<File>>) -> i32 {

    let mut sum = 0;

    loop {
        let r1 = lines.next();
        if r1.is_none() {
            break;
        }
        let r1 = r1.expect("No more lines for r1").expect("Error reading line for r1");
        let r2 = lines.next().expect("No more lines for r2").expect("Error reading line for r2");
        let r3 = lines.next().expect("No more lines for r3").expect("Error reading line for r3");

        let mut r1 : HashSet<char> = HashSet::from_iter(r1.chars());
        let r2 : HashSet<char> = HashSet::from_iter(r2.chars());
        let r3 : HashSet<char> = HashSet::from_iter(r3.chars());

        r1.retain( |chr| r2.contains(chr) );
        r1.retain( |chr| r3.contains(chr) );

        let x = r1.iter().next().expect("No common chars found");
        sum += to_priority(*x as i32);

    }
    
    return sum;

}

fn to_priority(item: i32) -> i32 {
    if item < 91 {
        i32::from(item) - 38
    } else {
        i32::from(item) - 96
    }
}

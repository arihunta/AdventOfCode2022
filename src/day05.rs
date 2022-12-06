use std::collections::HashMap;

use regex::Regex;

pub fn day05_01(lines: Vec<String>) -> String {
    let instruction_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();

    for line in lines {
        if line.contains("[") {
            let num_stacks = (line.len() + 1) / 4;

            for stack_idx in 0..num_stacks {
                let idx = 4 * stack_idx + 1;
                let chr = String::from(line[idx..(idx + 1)].trim());
                if !chr.is_empty() {
                    if !stacks.contains_key(&stack_idx) {
                        stacks.insert(stack_idx, Vec::new());
                    }

                    stacks.get_mut(&stack_idx).unwrap().insert(0, chr);
                }
            }
        } else if line.starts_with("move") {
            let hesx = instruction_pattern.captures(&line).unwrap();

            let num_elems = hesx[1].parse::<i32>().unwrap();
            let from = hesx[2].parse::<usize>().unwrap() - 1;
            let to = hesx[3].parse::<usize>().unwrap() - 1;

            for _i in 0..num_elems {
                let val = stacks.get_mut(&from).unwrap().pop().unwrap();
                stacks.get_mut(&to).unwrap().push(val);
            }
        }
    }

    (0..stacks.len())
        .map(|idx| {
            let v = stacks.get(&idx).unwrap();
            String::from(v.get(v.len() - 1).unwrap())
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn day05_02(lines: Vec<String>) -> String {
    let instruction_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();

    for line in lines {
        if line.contains("[") {
            let num_stacks = (line.len() + 1) / 4;

            for stack_idx in 0..num_stacks {
                let idx = 4 * stack_idx + 1;
                let chr = String::from(line[idx..(idx + 1)].trim());
                if !chr.is_empty() {
                    if !stacks.contains_key(&stack_idx) {
                        stacks.insert(stack_idx, Vec::new());
                    }

                    stacks.get_mut(&stack_idx).unwrap().insert(0, chr);
                }
            }
        } else if line.starts_with("move") {
            let hesx = instruction_pattern.captures(&line).unwrap();

            let num_elems = hesx[1].parse::<i32>().unwrap();
            let from = hesx[2].parse::<usize>().unwrap() - 1;
            let to = hesx[3].parse::<usize>().unwrap() - 1;

            let insert_idx = stacks.get(&to).unwrap().len();

            for _i in 0..num_elems {
                let val = stacks.get_mut(&from).unwrap().pop().unwrap();
                stacks.get_mut(&to).unwrap().insert(insert_idx, val);
            }
        }
    }

    (0..stacks.len())
        .map(|idx| {
            let v = stacks.get(&idx).unwrap();
            String::from(v.get(v.len() - 1).unwrap())
        })
        .collect::<Vec<String>>()
        .join("")
}

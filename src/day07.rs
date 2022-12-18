use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Directory {
    name: String,
    parent: String,
    files_size: u32
}

pub fn day07_01(lines: Vec<String>) -> u32 {

    // regex patterns
    let cd_pattern = Regex::new(r"^\$ cd (.+)$").unwrap();
    let dir_pattern = Regex::new(r"^dir (\w+)$").unwrap();
    let file_pattern = Regex::new(r"^(\d+) (.+)$").unwrap();

    // working variables
    let mut dirs : HashMap<String, Directory> = HashMap::new();
    let mut current_dir : String = String::from("");
    let mut dir_sizes : HashMap<String, u32> = HashMap::new();

    // add root directory
    let root_dir = Directory {
        name: String::from("/"),
        parent: String::from(""),
        files_size: 0
    };
    dirs.insert(String::from("/"), root_dir);

    // parse our input
    for line in lines {

        // changing directory -> change the value of `current_dir`
        if let Some(capture) = cd_pattern.captures_iter(&line).next() {
            match capture.get(1).unwrap().as_str() {
                ".." => current_dir = String::from(&dirs.get(&current_dir).unwrap().parent),
                "/" => current_dir = String::from("/"),
                dir => current_dir = current_dir + dir + "/",
            }
        }

        // new directory -> create the directory
        if let Some(capture) = dir_pattern.captures_iter(&line).next() {
            let new_dir = Directory {
                name: (String::from(&current_dir) + capture.get(1).unwrap().as_str() + "/"),
                parent: String::from(&current_dir),
                files_size: 0
            };
            dirs.insert(new_dir.name.clone(), new_dir);
        }

        // new file -> add its size to its parent
        if let Some(capture) = file_pattern.captures_iter(&line).next() {
            dirs.get_mut(&current_dir).unwrap().files_size += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
        }

    }

    for dir in dirs.values() {
        dir_sizes.entry(dir.name.clone()).or_insert(dir.files_size);
        for d2 in dirs.values() {
            if d2.parent.starts_with(&dir.name) {
                dir_sizes.entry(dir.name.clone()).and_modify(|it| *it += d2.files_size).or_insert(d2.files_size);
            }
        }
    }
    
    dir_sizes.values().filter(|it| *it < &100000).sum()

}

pub fn day07_02(lines: Vec<String>) -> u32 {

    // regex patterns
    let cd_pattern = Regex::new(r"^\$ cd (.+)$").unwrap();
    let dir_pattern = Regex::new(r"^dir (\w+)$").unwrap();
    let file_pattern = Regex::new(r"^(\d+) (.+)$").unwrap();

    // working variables
    let mut dirs : HashMap<String, Directory> = HashMap::new();
    let mut current_dir : String = String::from("");
    let mut dir_sizes : HashMap<String, u32> = HashMap::new();

    // add root directory
    let root_dir = Directory {
        name: String::from("/"),
        parent: String::from(""),
        files_size: 0
    };
    dirs.insert(String::from("/"), root_dir);

    // parse our input
    for line in lines {

        // changing directory -> change the value of `current_dir`
        if let Some(capture) = cd_pattern.captures_iter(&line).next() {
            match capture.get(1).unwrap().as_str() {
                ".." => current_dir = String::from(&dirs.get(&current_dir).unwrap().parent),
                "/" => current_dir = String::from("/"),
                dir => current_dir = current_dir + dir + "/",
            }
        }

        // new directory -> create the directory
        if let Some(capture) = dir_pattern.captures_iter(&line).next() {
            let new_dir = Directory {
                name: (String::from(&current_dir) + capture.get(1).unwrap().as_str() + "/"),
                parent: String::from(&current_dir),
                files_size: 0
            };
            dirs.insert(new_dir.name.clone(), new_dir);
        }

        // new file -> add its size to its parent
        if let Some(capture) = file_pattern.captures_iter(&line).next() {
            dirs.get_mut(&current_dir).unwrap().files_size += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
        }

    }

    for dir in dirs.values() {
        dir_sizes.entry(dir.name.clone()).or_insert(dir.files_size);
        for d2 in dirs.values() {
            if d2.parent.starts_with(&dir.name) {
                dir_sizes.entry(dir.name.clone()).and_modify(|it| *it += d2.files_size).or_insert(d2.files_size);
            }
        }
    }

    let needed_space = 30000000 - (70000000 - dir_sizes.get("/").expect("broken"));

    *dir_sizes.values().filter(|it| *it >= &needed_space).min().expect("msg")

}

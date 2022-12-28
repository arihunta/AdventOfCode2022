
use regex::Regex;

pub fn day10_01(lines: Vec<String>) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut cum_signal_strength = 0;

    let noop_pattern = Regex::new(r"^noop$").unwrap();
    let addx_pattern = Regex::new(r"^addx (-?[0-9]+)$").unwrap();

    lines.iter().for_each(|line| {
        if let Some(_) = noop_pattern.captures_iter(&line).next() {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                cum_signal_strength += cycle * x;
            }
        }
        if let Some(capture) = addx_pattern.captures_iter(&line).next() {
            let quantity: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                cum_signal_strength += cycle * x;
            }
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                cum_signal_strength += cycle * x;
            }
            x += quantity;
        }
    });

    cum_signal_strength
}

enum ProcessorAction {
    NoOp,
    Add(i32, i32),
}

pub fn day10_02(lines: Vec<String>) -> String {
    // regex consts
    let addx_pattern = Regex::new(r"^addx (-?[0-9]+)$").unwrap();

    // cpu state
    let mut cycle = 0;
    let mut x : i32 = 1;
    let mut current_instruction : Option<ProcessorAction> = None; 
    let mut buffer: [char; 40] = ['.'; 40];

    let mut lines_iter = lines.iter();
    loop {

        if current_instruction.is_none() {
            if let Some(next_instruction) = lines_iter.next() {
                if let Some(capture) = addx_pattern.captures_iter(&next_instruction).next() {
                    let quantity: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                    current_instruction = Some(ProcessorAction::Add(0, quantity));
                }
                else {
                    current_instruction = Some(ProcessorAction::NoOp);
                }
            } else {
                break;
            }
        }

        // process current pixel
        if (x >= (cycle as i32) - 1) && (x <= (cycle as i32) + 1) {
            buffer[cycle] = '#';
        }
        else {
            buffer[cycle] = '.';
        }

        // advance to next cycle, print buffer if needed
        cycle = (cycle + 1) % 40;
        if cycle == 0 {
            println!("{}", String::from_iter(buffer));
        }

        // process current instruction
        current_instruction = match current_instruction {
            Some(ProcessorAction::Add(0, value)) => Some(ProcessorAction::Add(1, value)),
            Some(ProcessorAction::Add(1, value)) => {x += value; None},
            _ => None,
        }
    }

    String::from("PLULKBZH")
}

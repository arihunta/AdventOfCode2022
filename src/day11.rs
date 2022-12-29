use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<u128>,
    operand: char,
    operation_param: i64,
    test_factor: i64,
    target_false: u32,
    target_true: u32,
    inspections: u64,
}

impl Monkey {}

fn load_barrel(input: String) -> HashMap<i32, Monkey> {
    let monkey_regex = r"Monkey (\d):\r\n  Starting items: (\d+(?:, \d+)*)\r\n  Operation: new = old (\+|\*) (\d+|old)\r\n  Test: divisible by (\d+)\r\n    If true: throw to monkey (\d+)\r\n    If false: throw to monkey (\d+)";
    let monkey_regex = Regex::new(monkey_regex).unwrap();
    monkey_regex
        .captures_iter(&input)
        .filter_map(|capture| {
            let groups = (
                capture.get(1),
                capture.get(2),
                capture.get(3),
                capture.get(4),
                capture.get(5),
                capture.get(6),
                capture.get(7),
            );
            match groups {
                (
                    Some(id),
                    Some(item_list),
                    Some(operand),
                    Some(operation_param),
                    Some(test_factor),
                    Some(target_true),
                    Some(target_false),
                ) => Some(Monkey {
                    id: id.as_str().parse().unwrap(),
                    items: item_list
                        .as_str()
                        .split(", ")
                        .map(|sgnmt| sgnmt.parse::<u128>().unwrap())
                        .collect(),
                    operand: operand.as_str().chars().next().unwrap(),
                    operation_param: operation_param.as_str().parse().unwrap_or(-1),
                    test_factor: test_factor.as_str().parse().unwrap(),
                    target_false: target_false.as_str().parse().unwrap(),
                    target_true: target_true.as_str().parse().unwrap(),
                    inspections: 0,
                }),
                _ => None,
            }
        })
        .map(|monkey| (monkey.id, monkey))
        .collect()
}

pub fn day11_01(input: String) -> i32 {
    let mut monkeys = load_barrel(input);

    for _ in 0..20 {
        (0..monkeys.len() as i32).for_each(|idx| {

            let (tf, tar_t, tar_f, operand, op_param) = monkeys
                .get(&idx)
                .map(|m| {
                    (
                        m.test_factor as u128,
                        m.target_true as i32,
                        m.target_false as i32,
                        m.operand,
                        m.operation_param,
                    )
                })
                .unwrap();

            let i2ms: Vec<u128> = monkeys
                .get(&idx)
                .unwrap()
                .items
                .iter()
                .cloned()
                .map(|item| {
                    let param = if op_param == -1 {
                        item
                    } else {
                        op_param as u128
                    };
                    match operand {
                        '+' => (item + param) / 3,
                        '*' => (item * param) / 3,
                        _ => item / 3,
                    }
                })
                .collect();

            monkeys.entry(tar_t).and_modify(|target| {
                target
                    .items
                    .append(&mut i2ms.iter().cloned().filter(|item| item % tf == 0).collect())
            });

            monkeys.entry(tar_f).and_modify(|target| {
                target
                    .items
                    .append(&mut i2ms.iter().cloned().filter(|item| item % tf != 0).collect())
            });

            monkeys.entry(idx).and_modify(|monk| {
                monk.inspections += monk.items.len() as u64;
                monk.items.clear();
            });
        });
    }

    let mut mk: Vec<u64> = monkeys.values().map(|mnk| mnk.inspections).collect();
    mk.sort();
    mk.reverse();

    (mk.get(0).unwrap() * mk.get(1).unwrap()) as i32
}

pub fn day11_02(input: String) -> i64 {
    let mut monkeys = load_barrel(input);

    let lcm : u128 = monkeys
        .values()
        .map(|m| m.test_factor)
        .reduce(|acc, a| if acc == 0 { a } else { acc * a })
        .unwrap_or(0).try_into().unwrap();

    for iter in 0..10000 {
        (0..monkeys.len() as i32).for_each(|idx| {

            let (tf, tar_t, tar_f, operand, op_param) = monkeys
                .get(&idx)
                .map(|m| {
                    (
                        m.test_factor as u128,
                        m.target_true as i32,
                        m.target_false as i32,
                        m.operand,
                        m.operation_param,
                    )
                })
                .unwrap();

            let i2ms: Vec<u128> = monkeys
                .get(&idx)
                .unwrap()
                .items
                .iter()
                .cloned()
                .map(|item| {
                    let param = if op_param == -1 {
                        item
                    } else {
                        op_param as u128
                    };
                    match operand {
                        '+' => item + param % lcm,
                        '*' => item * param % lcm,
                        _ => item,
                    }
                })
                .collect();

            monkeys.entry(tar_t).and_modify(|target| {
                target
                    .items
                    .append(&mut i2ms.iter().cloned().filter(|item| item % tf == 0).collect())
            });

            monkeys.entry(tar_f).and_modify(|target| {
                target
                    .items
                    .append(&mut i2ms.iter().cloned().filter(|item| item % tf != 0).collect())
            });

            monkeys.entry(idx).and_modify(|monk| {
                monk.inspections += monk.items.len() as u64;
                monk.items.clear();
            });
        });
    }

    let mut mk: Vec<u64> = monkeys.values().map(|mnk| mnk.inspections).collect();
    mk.sort();
    mk.reverse();

    (mk.get(0).unwrap() * mk.get(1).unwrap()) as i64
}

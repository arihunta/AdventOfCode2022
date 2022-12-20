use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn day08_01(mut lines: Vec<String>) -> u32 {
    let map: Vec<Vec<i32>> = lines
        .iter_mut()
        .map(|line| -> Vec<i32> { line.chars().map(|char| char as i32 - 0x30).collect() })
        .collect();

    let y_max = map.len();
    let x_max = map[0].len();

    let mut positions: HashSet<(usize, usize)> = (0..y_max)
        .flat_map(|y_coord| -> Vec<(usize, usize)> {
            (0..x_max).map(|x_coord| (x_coord, y_coord)).collect()
        })
        .collect();

    for y in 0..y_max {
        let mut tallest: i32 = -1;
        for x in 0..x_max {
            if map[x][y] > tallest {
                tallest = map[x][y];
                positions.remove(&(x, y));
            }
        }
        tallest = -1;
        for x in (0..x_max).rev() {
            if map[x][y] > tallest {
                tallest = map[x][y];
                positions.remove(&(x, y));
            }
        }
    }

    for x in 0..x_max {
        let mut tallest = -1;
        for y in 0..y_max {
            if map[x][y] > tallest {
                tallest = map[x][y];
                positions.remove(&(x, y));
            }
        }
        tallest = -1;
        for y in (0..y_max).rev() {
            if map[x][y] > tallest {
                tallest = map[x][y];
                positions.remove(&(x, y));
            }
        }
    }

    ((x_max * y_max) - positions.len()) as u32
}

pub fn day08_02(mut lines: Vec<String>) -> u32 {
    let mut slices: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    let mut x_max: i32 = -1;
    let mut y_max: i32 = -1;

    lines.iter_mut().enumerate().for_each(|(y, line)| -> () {
        line.chars().enumerate().for_each(|(x, val)| {
            let val = val as i32 - 0x30;
            (0..=val).for_each(|slice| {
                slices
                    .entry(slice)
                    .or_insert(HashSet::new())
                    .insert((x as i32, y as i32));
            });
            x_max = max(x_max, x as i32);
        });
        y_max = max(y_max, y as i32);
    });

    let mut ans: i32 = -1;

    slices.values().for_each(|slice| {
        slice.iter().for_each(|(x, y)| {
            let mut up: i32 = *y;
            let mut left: i32 = *x;
            let mut down = y_max - y;
            let mut right = x_max - x;
            slice.iter().for_each(|(x_p, y_p)| {
                if x_p == x && y_p > y {
                    down = min(down, y_p - y);
                } else if x_p == x && y_p < y {
                    up = min(up, y - y_p);
                } else if x_p > x && y_p == y {
                    right = min(right, x_p - x);
                } else if x_p < x && y_p == y {
                    left = min(left, x - x_p);
                }
            });
            ans = max(ans, up * down * left * right);
        })
    });

    ans as u32
}

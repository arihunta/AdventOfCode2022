use std::collections::HashSet;

pub fn day09_01(mut lines: Vec<String>) -> u32 {
    let mut t = (0, 0);
    let mut h = (0, 0);
    let mut history: HashSet<(i32, i32)> = HashSet::new();

    history.insert(t);

    lines
        .iter_mut()
        .map(|line| -> (char, i32) {
            let mut line = line.chars();
            let dir = line.next().expect("");
            line.next();
            let steps: String = line.collect();
            (dir, steps.parse().unwrap())
        })
        .for_each(|(dir, steps)| {
            for _ in 0..steps {
                match dir {
                    'U' => h = (h.0, h.1 + 1),
                    'D' => h = (h.0, h.1 - 1),
                    'L' => h = (h.0 - 1, h.1),
                    'R' => h = (h.0 + 1, h.1),
                    _ => (),
                }
                t = follow_nodes(&h, &t);
                history.insert(t);
            }
        });

    history.len() as u32
}

pub fn day09_02(mut lines: Vec<String>) -> u32 {
    let mut nodes : [(i32, i32); 10] = [(0,0); 10];
    let mut history: HashSet<(i32, i32)> = HashSet::new();

    history.insert(nodes[8]);

    lines
        .iter_mut()
        .map(|line| -> (char, i32) {
            let mut line = line.chars();
            let dir = line.next().expect("");
            line.next();
            let steps: String = line.collect();
            (dir, steps.parse().unwrap())
        })
        .for_each(|(dir, steps)| {
            for _ in 0..steps {
                match dir {
                    'U' => nodes[0] = (nodes[0].0, nodes[0].1 + 1),
                    'D' => nodes[0] = (nodes[0].0, nodes[0].1 - 1),
                    'L' => nodes[0] = (nodes[0].0 - 1, nodes[0].1),
                    'R' => nodes[0] = (nodes[0].0 + 1, nodes[0].1),
                    _ => (),
                }
                (1..10).for_each(|idx| nodes[idx] = follow_nodes(&nodes[idx - 1], &nodes[idx]));
                history.insert(nodes[9]);
            }
        });

    history.len() as u32
}

fn follow_nodes(h: &(i32, i32), t: &(i32, i32)) -> (i32, i32) {
    match (h.0 - t.0, h.1 - t.1) {
        (-2, 2) => (t.0 - 1, t.1 + 1),
        (-1, 2) => (t.0 - 1, t.1 + 1),
        (0, 2) => (t.0, t.1 + 1),
        (1, 2) => (t.0 + 1, t.1 + 1),
        (2, 2) => (t.0 + 1, t.1 + 1),

        (-2, -2) => (t.0 - 1, t.1 - 1),
        (-1, -2) => (t.0 - 1, t.1 - 1),
        (0, -2) => (t.0, t.1 - 1),
        (1, -2) => (t.0 + 1, t.1 - 1),
        (2, -2) => (t.0 + 1, t.1 - 1),
        
        (-2, -1) => (t.0 - 1, t.1 - 1),
        (-2, 0) => (t.0 - 1, t.1),
        (-2, 1) => (t.0 - 1, t.1 + 1),
        
        (2, -1) => (t.0 + 1, t.1 - 1),
        (2, 0) => (t.0 + 1, t.1),
        (2, 1) => (t.0 + 1, t.1 + 1),
        _ => (t.0, t.1)
    }
}

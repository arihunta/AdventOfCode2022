use std::{io::{Lines, BufReader}, fs::File};

pub fn day02_01(lines: Lines<BufReader<File>>) -> i32 {

    let mut score : i32 = 0;

    for round in lines {
        let round = round.unwrap();
        let opponent_play = &round[0..1];
        let my_play = &round[2..];
        score += match opponent_play {
            "A" => match my_play {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                _ => 0
            },
            "B" => match my_play {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => 0
            },
            "C" => match my_play {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                _ => 0
            },
            _ => 0
        };
    }

    return score;

}

pub fn day02_02(lines: Lines<BufReader<File>>) -> i32 {

    let mut score : i32 = 0;

    for round in lines {
        let round = round.unwrap();
        let opponent_play = &round[0..1];
        let my_play = &round[2..];
        score += match opponent_play {
            "A" => match my_play {
                "X" => 0 + 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => 0
            },
            "B" => match my_play {
                "X" => 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => 0
            },
            "C" => match my_play {
                "X" => 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => 0
            },
            _ => 0
        };
    }

    return score;

}

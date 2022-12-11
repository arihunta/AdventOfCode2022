
pub fn day06_01(line: String) -> i32 {
    ((3..line.len())
        .find(|idx| -> bool {
            line.as_bytes()[idx-3] != line.as_bytes()[idx-2]
            && line.as_bytes()[idx-3] != line.as_bytes()[idx-1]
            && line.as_bytes()[idx-3] != line.as_bytes()[idx-0]
            && line.as_bytes()[idx-2] != line.as_bytes()[idx-1]
            && line.as_bytes()[idx-2] != line.as_bytes()[idx-0]
            && line.as_bytes()[idx-1] != line.as_bytes()[idx-0]
        })
        .unwrap_or(0) + 1) as i32
}

pub fn day06_02(line: String) -> i32 {

    let mut letter_counts : [u8; 26] = [0; 26];
    let message_marker_length = 14;
    
    (0..message_marker_length)
        .for_each(|idx| -> () {
            letter_counts[(&line.as_bytes()[idx] - 97) as usize] += 1;
        });

    ((message_marker_length..line.len())
        .find(|&idx| -> bool {
            letter_counts[(&line.as_bytes()[idx] - 97) as usize] += 1;
            letter_counts[(&line.as_bytes()[idx - message_marker_length] - 97) as usize] -= 1;
            letter_counts.iter().position(|count| count > &1).is_none()
        })
        .unwrap_or(0) + 1) as i32

}

use std::io::stdin;

pub fn fn02_02() {
    let safe_count = stdin()
        .lines()
        .filter(|line| {
            let seq = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if is_valid_sequence(seq.clone()) {
                return true;
            }

            (0..(seq.len())).any(|i| is_valid_sequence([&seq[..i], &seq[i + 1..]].concat()))
        })
        .count();

    println!("{}", safe_count);
}

fn is_valid_sequence(seq: Vec<i32>) -> bool {
    seq.windows(2)
        .try_fold(0, |prev_diff, pair| {
            let curr = pair[0];
            let next = pair[1];
            let diff = next - curr;

            if is_valid_diff(diff, prev_diff) {
                Ok(diff)
            } else {
                Err(())
            }
        })
        .is_ok()
}

fn is_valid_diff(diff: i32, prev_diff: i32) -> bool {
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    diff * prev_diff >= 0
}

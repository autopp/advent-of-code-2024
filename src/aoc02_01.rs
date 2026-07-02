use std::io::stdin;

pub fn fn02_01() {
    let safe_count = stdin()
        .lines()
        .filter(|line| {
            line.as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
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
        })
        .count();

    println!("{}", safe_count);
}

fn is_valid_diff(diff: i32, prev_diff: i32) -> bool {
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    diff * prev_diff >= 0
}

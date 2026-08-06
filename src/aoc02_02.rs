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

            match find_invalid_index(seq.clone()) {
                Some(index) => [0, index, index + 1]
                    .into_iter()
                    .any(|i| find_invalid_index([&seq[..i], &seq[i + 1..]].concat()).is_none()),
                None => true,
            }
        })
        .count();

    println!("{}", safe_count);
}

fn find_invalid_index(seq: Vec<i32>) -> Option<usize> {
    seq.windows(2)
        .enumerate()
        .try_fold(0, |prev_diff, (index, pair)| {
            let curr = pair[0];
            let next = pair[1];
            let diff = next - curr;

            if is_valid_diff(diff, prev_diff) {
                Ok(diff)
            } else {
                Err(index)
            }
        })
        .err()
}

fn is_valid_diff(diff: i32, prev_diff: i32) -> bool {
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    diff * prev_diff >= 0
}

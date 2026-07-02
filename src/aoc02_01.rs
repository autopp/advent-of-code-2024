use std::io::stdin;

pub fn fn02_01() {
    let safe_count = stdin()
        .lines()
        .filter(|line| {
            let mut numbers = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap());

            let first = numbers.next().unwrap();
            let second = numbers.next().unwrap();
            let first_diff = second - first;
            if first_diff.abs() < 1 || first_diff.abs() > 3 {
                return false;
            }

            let should_be_acc = first_diff > 0;

            numbers
                .try_fold(second, |prev, curr| {
                    if is_valid_sequence(prev, curr, should_be_acc) {
                        Ok::<_, ()>(curr)
                    } else {
                        Err(())
                    }
                })
                .is_ok()
        })
        .count();

    println!("{}", safe_count);
}

fn is_valid_sequence(prev: i32, curr: i32, should_be_acc: bool) -> bool {
    let diff = curr - prev;
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    (diff > 0) == should_be_acc
}

use std::io::{Read, stdin};

pub fn fn03_02() {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf).unwrap();
    let input = std::str::from_utf8(&buf).unwrap();

    let re = regex::Regex::new(r"(mul\((?P<left>\d{1,3}),(?P<right>\d{1,3})\))|do\(\)|don't\(\)")
        .unwrap();

    let mut enabled = true;
    let result = re
        .captures_iter(input)
        .filter_map(|cap| {
            if cap[0].starts_with("mul") {
                if enabled {
                    Some(cap["left"].parse::<i32>().unwrap() * cap["right"].parse::<i32>().unwrap())
                } else {
                    None
                }
            } else if cap[0] == *"do()" {
                enabled = true;
                None
            } else if cap[0] == *"don't()" {
                enabled = false;
                None
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("{}", result);
}

use std::io::{Read, stdin};

pub fn fn03_01() {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf).unwrap();
    let input = std::str::from_utf8(&buf).unwrap();

    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = re
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
        .sum::<i32>();

    println!("{}", result);
}

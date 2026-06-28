use std::io::stdin;

pub fn fn01_01() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    stdin().lines().for_each(|line| {
        let (l, r) = parse_line(&line.unwrap());
        left.push(l);
        right.push(r);
    });

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());
    println!("{}", result);
}

fn parse_line(line: &str) -> (i32, i32) {
    let parsed = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (parsed[0], parsed[1])
}

use std::{collections::HashMap, io::stdin};

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    stdin().lines().for_each(|line| {
        let (l, r) = parse_line(&line.unwrap());
        left.push(l);
        *right.entry(r).or_insert(0) += 1;
    });

    let result = left
        .iter()
        .map(|l| l * right.get(l).unwrap_or(&0))
        .sum::<i32>();
    println!("{}", result);
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut iter = line.split_whitespace();

    (
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
    )
}

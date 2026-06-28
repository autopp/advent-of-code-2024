mod aoc01_01;
mod aoc01_02;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <target>", args[0]);
        std::process::exit(1);
    }
    let target = &args[1];

    match target.as_str() {
        "01_01" => {
            aoc01_01::fn01_01();
        }
        "01_02" => {
            aoc01_02::fn01_02();
        }
        _ => {
            eprintln!("Unknown target: {}", target);
            std::process::exit(1);
        }
    }
}

mod aoc01_01;
mod aoc01_02;
mod aoc02_01;
mod aoc02_02;
mod aoc03_01;

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
        "02_01" => {
            aoc02_01::fn02_01();
        }
        "02_02" => {
            aoc02_02::fn02_02();
        }
        "03_01" => {
            aoc03_01::fn03_01();
        }
        _ => {
            eprintln!("Unknown target: {}", target);
            std::process::exit(1);
        }
    }
}

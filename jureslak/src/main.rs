use std::fs::File;
use std::io::{BufReader, BufRead};

mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
use common::Part;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day : usize = args.get(1).expect("Supply the day as the first argument.")
        .parse().expect("Cannot parse as an integer.");
    let part : i32 = args.get(2).expect("Supply the part as the second argument.")
        .parse().expect("Cannot parse as an integer.");

    let default = format!("input/day{:02}.in", day);
    let file= args.get(3).unwrap_or(&default);

    let data : Vec<String> = BufReader::new(
        File::open(file).expect("Failed opening file.")
    ).lines().map(|l| l.expect("Failed reading line.")).collect();

    let solutions = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
    ];

    if !(part == 1 || part == 2) { panic!("Invalid part!"); }
    solutions.get(day-1).expect("Invalid day.")(
        &data,
        if part == 1 { Part::First } else { Part::Second }
    );
}
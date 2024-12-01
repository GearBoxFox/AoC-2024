use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

mod day1;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("please choose a day and part");
        return Err(ErrorKind::Other.into());
    }

    let day = args[1].parse::<u8>().unwrap_or(1);
    let part2 = args[2].parse::<bool>().unwrap_or(false);
    let example = args[3].parse::<bool>().unwrap_or(false);

    let day_path: String;

    if example {
        day_path = format!("./inputs/day{}example", day);
    } else {
        day_path = format!("./inputs/day{}", day);
    }

    let mut file = File::open(day_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let solution = match day {
        1 => if !part2 {day1::solve1(&contents)} else {day1::solve2(&contents)}
        _ => 0
    };

    println!("Day {}", day);
    if part2 { println!("Part 2") } else { println!("Part 1") }
    println!("Solution: {}", solution);

    Ok(())
}

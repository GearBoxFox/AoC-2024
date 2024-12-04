extern crate regex;

use regex::Regex;

pub fn solve1(input: &String, part2: bool) -> isize {
    let regex: Regex;
    regex = Regex::new(r"(?P<do>(do\(\))|(don't\(\)))|(mul\((?P<num1>\d+),(?P<num2>\d+)\))").unwrap();

    let mut sum: isize = 0;

    // result will be an iterator of tuples containing start and end indices

    let result = regex.captures_iter(input);

    let mut do_mult = true;
    for capture in result {
        // println!("{} , {}", &capture["num1"], &capture["num2"]);
        if part2  && capture.name("do") != None {

            // let mult = &capture["do"];//.parse::<String>().unwrap_or("do()".parse().unwrap());
            if &capture["do"] == "don't()" {
                do_mult = false;
            } else if &capture["do"] == "do()" {
                do_mult = true;
            }
        }

        if do_mult && capture.name("num1") != None && capture.name("num2") != None {
            sum += &capture["num1"].parse::<isize>().unwrap_or(0) * &capture["num2"].parse::<isize>().unwrap_or(0);
        }
    }

    sum
}

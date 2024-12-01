pub fn solve1(input: &String) -> isize {
    let lines = input.lines();
    let mut sum: i32 = 0;

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // get the numbers into the vecs
    for line in lines {
        let mut iter = line.split_ascii_whitespace();
        let num1 = iter.next().unwrap().trim().parse::<i32>().unwrap();
        let num2 = iter.next().unwrap().trim().parse::<i32>().unwrap();

        left.push(num1);
        right.push(num2);
    }

    // sort the vecs from lowest to greatest
    left.sort();
    right.sort();

    // assume both vecs are same size
    for i in 0..left.len() {
        sum += i32::abs(left.get(i).unwrap() - right.get(i).unwrap())
    }

    sum as isize
}

pub fn solve2(input: &String) -> isize {
    let lines = input.lines();
    let mut sum: i32 = 0;

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // get the numbers into the vecs
    for line in lines {
        let mut iter = line.split_ascii_whitespace();
        let num1 = iter.next().unwrap().trim().parse::<i32>().unwrap();
        let num2 = iter.next().unwrap().trim().parse::<i32>().unwrap();

        left.push(num1);
        right.push(num2);
    }

    for i in 0..left.len() {
        let sim = right.iter().filter(|&&x| x == *left.get(i).unwrap()).count();
        sum += *left.get(i).unwrap() * sim as i32;
    }

    sum as isize
}
fn is_good<'a>(mut iter: impl Iterator<Item = &'a i32>) -> bool {
    let mut ascending = None;
    let mut a = iter.next().unwrap();

    for b in iter {
        if a == b || a.abs_diff(*b) > 3{
            return false;
        }

        match ascending {
            None => ascending = Some(a > b),
            Some(true) => if a < b {return false;},
            Some(false) => if a > b {return false;},
        }

        a = b;
    }

    true
}

pub fn solve1(input: &String, _day_2: bool) -> isize {
    let lines = input.lines();
    let mut safe: i32 = 0;

    for line in lines {
        let chars = line.chars().map(|x| x.to_digit(10));
        let mut nums: Vec<i32> = Vec::new();
        let mut temp: i32 = 0;

        for char in chars {
            if char.is_some() {
                temp = (temp * 10) + char.unwrap() as i32;
            } else {
                nums.push(temp);
                temp = 0;
            }
        }

        nums.push(temp);

        if is_good(nums.iter()) {
            safe += 1;
        } else {
            for i in 0..nums.len() {
                let mut temp = nums.clone();
                temp.remove(i);
                if is_good(temp.iter()) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    safe as isize
}
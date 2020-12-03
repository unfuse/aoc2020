use std::fs;
use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("src/bin/input02.txt").expect("Could not read file");
    let mut total = 0;

    let assign = Regex::new("^(\\d+)-(\\d+) (\\w): (\\w+)$").unwrap();

    for line in input.lines() {
        for captures in assign.captures_iter(line) {
            let lower: usize = captures[1].parse::<usize>().unwrap();
            let upper: usize = captures[2].parse::<usize>().unwrap();
            let target: char = captures[3].parse::<char>().unwrap();
            let password: &str = &captures[4];

            // println!("{}-{} {}: {}", lower, upper, target, password);

            let check1 = password.chars().nth(&lower-1).unwrap() == target;
            let check2 = password.chars().nth(&upper-1).unwrap() == target;

            // println!("{} {} -> {} {} -> {} {}", password, target, lower-1, upper-1, check1, check2);

            if (check1 || check2) && (check1 != check2) {
                total += 1;
            }

            // Part 1
            // let mut letter_count = 0;
            // for letter in password.chars() {
            //     if letter == target {
            //         letter_count += 1;
            //     }
            // }
            // if letter_count >= lower && letter_count <= upper {
            //     total += 1;
            // }
        }
    }

    println!("{}", total);
}
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/bin/input08.txt").expect("Could not read file");
    let inst: Vec<(String, isize)> = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.trim().split_whitespace().collect();
            (split[0].to_string(), split[1].parse::<isize>().unwrap())
        })
        .collect();

    // Part 1
    println!("{:?}", run_code(&inst));

    // Part 2
    for i in 0..inst.len() {
        let mut inst_copy = inst.clone();
        let x = &inst_copy[i];
        let instruction = &*x.0;
        let number = x.1;

        match instruction {
            "jmp" => inst_copy[i] = (String::from("nop"), number),
            "nop" => inst_copy[i] = (String::from("jmp"), number),
            _ => continue,
        }

        if let (true, acc) = run_code(&inst_copy) {
            println!("acc: {} => termed swapping instruction {}", acc, i);
            break;
        }
    }
}

fn run_code(inst: &[(String, isize)]) -> (bool, isize) {
    let mut cur: usize = 0;
    let mut acc: isize = 0;
    let mut seen_lines: HashSet<usize> = HashSet::new();

    while cur < inst.len() && seen_lines.insert(cur) {
        let x = &inst[cur];
        let instruction = &*x.0;
        let number = x.1;

        match instruction {
            "acc" => {
                acc += number;
                cur += 1;
            }
            "jmp" => {
                let number_usize = usize::try_from(number.abs()).unwrap();
                if number > 0 {
                    cur += number_usize;
                } else if number_usize <= cur {
                    cur -= number_usize;
                } else {
                    return (false, acc);
                }
            }
            "nop" => cur += 1,
            _ => panic!("unknown instruction {}", instruction),
        }
    }

    (cur >= inst.len(), acc)
}

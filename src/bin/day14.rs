use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/input14.txt").unwrap();

    println!("part 1: {}", version_1(input.clone()));
    println!("part 2: {}", version_2(input));
}

fn version_1(input: String) -> u64 {
    let data = input.lines();
    let mut memory: HashMap<usize, u64> = HashMap::new();

    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = 0;

    let regex: Regex = Regex::new("mem\\[(\\d+)] = (\\d+)").unwrap();

    for line in data {
        if line.starts_with("mask") {
            let mask_str: &str = line.split(" = ").last().unwrap();
            and_mask = u64::from_str_radix(mask_str.replace("X", "1").as_str(), 2).unwrap();
            or_mask = u64::from_str_radix(mask_str.replace("X", "0").as_str(), 2).unwrap();
        } else if line.starts_with("mem") {
            for captures in regex.captures_iter(line) {
                let addr: usize = captures[1].parse::<usize>().unwrap();
                let val: u64 = captures[2].parse::<u64>().unwrap();
                let adj_val: u64 = (val & and_mask) | or_mask;
                memory.insert(addr, adj_val);
            }
        } else {
            panic!();
        }
    }
    memory.values().sum::<u64>()
}

fn version_2(input: String) -> u64 {
    let data = input.lines();
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask_str: &str = "";

    let regex: Regex = Regex::new("mem\\[(\\d+)] = (\\d+)").unwrap();

    for line in data {
        if line.starts_with("mask") {
            mask_str = line.split(" = ").last().unwrap();
        } else if line.starts_with("mem") {
            for captures in regex.captures_iter(line) {
                let addr: u64 = captures[1].parse::<u64>().unwrap();
                let val: u64 = captures[2].parse::<u64>().unwrap();

                for adj_addr in all_perms(mask_str, addr) {
                    memory.insert(adj_addr as usize, val);
                }
            }
        } else {
            panic!();
        }
    }
    memory.values().sum::<u64>()
}

fn all_perms(mask: &str, addr: u64) -> Vec<u64> {
    let num_x = mask.chars().filter(|c| *c == 'X').count();
    // Cause it's where all my X's live
    let texas = mask.match_indices('X');

    let or_mask = u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap();
    let addr_str = format!("{:0>36b}", addr | or_mask);

    let mut resp: Vec<u64> = Vec::new();

    for n in permute(num_x) {
        let mut addr_vec: Vec<char> = addr_str.clone().chars().collect();
        for (i, (xi, _)) in texas.clone().enumerate() {
            addr_vec[xi] = n[i];
        }
        resp.push(
            u64::from_str_radix(addr_vec.into_iter().collect::<String>().as_str(), 2).unwrap(),
        )
    }
    resp
}

fn permute(len: usize) -> Vec<Vec<char>> {
    (0..2_usize.pow(len as u32))
        .into_iter()
        .map(|i| format!("{:b}", i))
        .map(|s| format!("{:0>1$}", s, len))
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

// From Mask    = 1XXXX0X
// Example  11  = 0001011
// Example 101  = 1100101
// Example   0  = 0000000
//
// Force 0 mask = 1111101
// M | I | R
// 0 | 0 | 0
// 0 | 1 | 0
// 1 | 0 | 0
// 1 | 1 | 1
// (== AND TRUE mask?)
//
// Example results = 0001001
// Example results = 1100101
// Example results = 0000000
//
// then...
//
// Force 1 mask = 1000000
// M | I | R
// 0 | 0 | 0
// 0 | 1 | 1
// 1 | 0 | 1
// 1 | 1 | 1
// (== OR TRUE mask?)
//
// Example results = 1001001 ==? 1001001 TRUE
// Example results = 1100101 ==? 1100101 TRUE
// Example results = 1000000 ==? 1000000 TRUE

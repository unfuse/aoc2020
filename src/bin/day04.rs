use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("src/bin/input04.txt").expect("Could not read file");
    let total: usize;

    let pairs = Regex::new("(\\S+)").unwrap();
    let keyval = Regex::new("(\\w+):(.+)").unwrap();
    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    for obj in input.split("\n\n") {
        let mut map: HashMap<String, String> = HashMap::new();
        for pair in pairs.captures_iter(obj) {
            for derps in keyval.captures_iter(&pair[1]) {
                map.insert(String::from(&derps[1]), String::from(&derps[2]));
            }
        }
        passports.push(map);
    }

    total = passports.iter()
        .filter(|x| x.keys().count() >= 7)
        .filter(|x| {
            check_byr(x) && check_ecl(x) && check_eyr(x) && check_hcl(x) &&
                check_hgt(x) && check_iyr(x) && check_pid(x)
        })
        .count();

    // Part 1
    // total = passports.iter()
    //     .filter(|x| { x.keys().count() == 8 || x.keys().count() == 7 && x.get("cid") == None})
    //     .count();

    println!("{}", total);
}

fn check_byr(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "byr", |val| val_in_range(val, 1920, 2002))
}

fn check_iyr(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "iyr", |val| val_in_range(val, 2010, 2020))
}

fn check_eyr(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "eyr", |val| val_in_range(val, 2020, 2030))
}

fn check_hgt(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "hgt", |val| {
        if val.contains("in") {
            val_in_range(&val.replace("in", ""), 59, 76)
        }
        else {
            val_in_range(&val.replace("cm", ""), 150, 193)
        }
    })
}

fn check_hcl(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "hcl", |val| Regex::new("^#[\\da-f]{6}$").unwrap().is_match(val))
}

fn check_ecl(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "ecl", |val| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val))
}

fn check_pid(map: &HashMap<String, String>) -> bool {
    check_map_val(map, "pid", |val| val.len() == 9)
}

fn check_map_val(map: &HashMap<String, String>, key: &str, fun: impl Fn(&str) -> bool) -> bool {
    if let Some(val) = map.get(key) {
        return fun(val);
    }
    false
}

fn val_in_range(val: &str, lower: usize, upper: usize) -> bool {
    let num = val.parse::<usize>().unwrap();
    // println!("    checking val {} between {}-{}", val, lower, upper);

    num >= lower && num <= upper
}
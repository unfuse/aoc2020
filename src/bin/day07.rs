use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

struct BagRule {
    qty: usize,
    bag: String
}

fn main() {
    let input: String = fs::read_to_string("src/bin/input07.txt").expect("Could not read file");
    let my_bag: String = String::from("shiny gold");
    let mut map: HashMap<String, Vec<BagRule>> = HashMap::new();

    let val_splitter = Regex::new("(\\d+) (\\D+) [bags]+[.,\\s]").unwrap();

    for line in input.lines() {
        let split: Vec<&str> = line.split(" bags contain ").collect();
        let key = split[0];
        let val = split[1];
        let mut values: Vec<BagRule> = Vec::new();

        if val != "no other bags." {
            for captures in val_splitter.captures_iter(val) {
                let qty: usize = captures[1].parse::<usize>().unwrap();
                let bag: String = captures[2].to_string();
                values.push(BagRule{qty, bag});
            }
        }
        map.insert(key.to_string(), values);
    }

    // Part 1
    let part1 = find_containers_of_value(&map, &my_bag);
    println!("{} => {:?}",  part1.len(), part1);

    // Part 2
    println!("{}", calculate_qty_bags_contained(&map,  &my_bag));
}

fn find_containers_of_value(map: &HashMap<String, Vec<BagRule>>, value: &str) -> HashSet<String> {
    let mut workings: HashSet<String> = HashSet::new();
    let mut answers: HashSet<String> = HashSet::new();

    workings.insert(value.to_string());

    loop {
        let mut next_workings: HashSet<String> = HashSet::new();
        for working in workings {
            for (key, value) in map.iter() {
                let just_bags: Vec<String> = value.iter().map(|x| String::from(&x.bag)).collect();
                if just_bags.contains(&working) && !answers.contains(key) {
                    next_workings.insert(key.to_string());
                    answers.insert(key.to_string());
                }
            }
        }

        if next_workings.is_empty() {
            break;
        }
        else {
            workings = next_workings;
        }
    }

    return answers;
}

fn calculate_qty_bags_contained(map: &HashMap<String, Vec<BagRule>>, value: &str) -> usize {
    map.get(value).unwrap().iter()
        .map(|bag_rule| bag_rule.qty * (1 + calculate_qty_bags_contained(map, &bag_rule.bag)))
        .sum()
}
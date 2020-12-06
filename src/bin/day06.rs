use std::fs;
use std::collections::HashSet;

fn main() {
    let input: String = fs::read_to_string("src/bin/input06.txt").expect("Could not read file");

    // Part 1
    let total: usize = input.split("\n\n")
        .map(|group| group.lines()
            .flat_map(|line| line.chars())
            .collect::<HashSet<char>>()
        )
        .map(|set| set.len())
        .sum();

    println!("{}", total);

    let ascii_lower: [char; 26] = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y',
        'z',
    ];
    let ascii_lower_set: HashSet<char> = ascii_lower.iter().map(|c| *c).collect();

    // Part 2
    let total2: usize = input.split("\n\n")
        .map(|group| group.lines()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .fold(ascii_lower_set.clone(), |acc, x| acc.intersection(&x).map(|c| *c).collect())
        )
        .map(|set| set.len())
        .sum();

    println!("{}", total2)
}
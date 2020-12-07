use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/bin/input01.txt").expect("Could not read file");
    let items: Vec<i32> = input
        .lines()
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .collect();

    let mut total = 0;
    for (i, num1) in items.iter().enumerate() {
        for (j, num2) in items.iter().skip(i + 1).enumerate() {
            // Part 2
            for num3 in items.iter().skip(j + 1) {
                if num1 + num2 + num3 == 2020 {
                    total = num1 * num2 * num3;
                }
            }

            // Part 1
            // if num1 + num2 == 2020 {
            //     total = num1 * num2;
            // }
        }
    }

    println!("{}", total);
}

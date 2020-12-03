use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/bin/input01.txt").expect("Could not read file");
    let items: Vec<i32> = input
        .lines()
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .collect();

    let len = items.len();
    let mut total = 0;
    for i in 0..len {
        let num1 = items[i];
        for j in i+1..len {
            let num2 = items[j];
            for k in j+1..len {
                let num3 = items[k];

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
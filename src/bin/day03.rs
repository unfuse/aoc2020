use std::fs;
use aoc2020::data::Data;

fn main() {
    let input: String = fs::read_to_string("src/bin/input03.txt").expect("Could not read file");
    let data = Data::new(&input);
    let mut total = 1;

    // Part 2
    for (x, y) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        total *= test_slope(*x, *y, &data);
    }

    // Part 1
    // total = test_slope(3, 1, &data);

    println!("{}", total);
}

fn test_slope(del_x: usize, del_y: usize, data: &Data) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count: usize = 0;

    loop {
        x = (x + del_x) % data.width();
        y += del_y;

        if y >= data.height() {
            break count;
        }

        if data.is(x, y, '#') {
            count += 1
        }
    }
}
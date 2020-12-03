use std::fs;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("src/bin/input03.txt").expect("Could not read file");
    let mut total = 1;
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut height: usize = 0;
    let mut width: usize = 0;

    for (i, line) in input.lines().enumerate() {
        width = line.len();
        for (j, c) in line.chars().enumerate() {
            map.insert((j, i), c);
        }
        height += 1;
    }

    for (x, y) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        total = total * test_slope(x, y, &map, width, height)
    }

    // Part 1
    // total = test_slope(3, 1, &map, width, height);

    println!("{}", total);
}

fn test_slope(del_x: usize, del_y: usize, map: &HashMap<(usize, usize), char>, width: usize, height: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count: usize = 0;

    return loop {
        x = (x + del_x) % (width);
        y = y + del_y;

        if y >= height {
            break count;
        }

        let c = map.get(&(x, y)).unwrap();
        if *c == '#' {
            count += 1;
        }
    }
}
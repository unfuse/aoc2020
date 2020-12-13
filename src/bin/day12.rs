use std::fs;
use std::mem::swap;

fn main() {
    let data: String = fs::read_to_string("src/bin/input12.txt").unwrap();
    let instructions: Vec<&str> = data.lines().collect();

    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut way_x = 10;
    let mut way_y = 1;

    for inst in instructions {
        let code: char = *inst[..1].chars().collect::<Vec<char>>().get(0).unwrap();
        let num: usize = inst[1..].parse().unwrap();

        match code {
            'N' | 'E' | 'S' | 'W' => {
                let (new_x, new_y) = adjust_by_dir(way_x, way_y, code, num);
                way_x = new_x;
                way_y = new_y;
            }
            'R' | 'L' => {
                let (new_x, new_y) = rotate(way_x, way_y, code, num);
                way_x = new_x;
                way_y = new_y;
            }
            'F' => {
                let (new_x, new_y) = adjust(ship_x, ship_y, way_x, way_y, num);
                ship_x = new_x;
                ship_y = new_y;
            }
            _ => panic!(),
        }
    }

    println!("{}", ship_x.abs() + ship_y.abs());
}

fn adjust(cur_x: isize, cur_y: isize, mag_x: isize, mag_y: isize, num: usize) -> (isize, isize) {
    (
        cur_x + (mag_x * num as isize),
        cur_y + (mag_y * num as isize),
    )
}

fn rotate(cur_x: isize, cur_y: isize, rot: char, num: usize) -> (isize, isize) {
    let mut last_x = cur_x;
    let mut last_y = cur_y;

    let rot_amt = match rot {
        'L' => (360 - num) as usize,
        'R' => num,
        _ => panic!("rotating a weird direction: {}", rot),
    } / 90;

    for _ in 0..rot_amt {
        swap(&mut last_x, &mut last_y);
        last_y *= -1;
    }

    (last_x, last_y)
}

// Part 1, and move waypoint for part 2
fn adjust_by_dir(x: isize, y: isize, dir: char, num: usize) -> (isize, isize) {
    match dir {
        'N' => (x, y + num as isize),
        'S' => (x, y - num as isize),
        'E' => (x + num as isize, y),
        'W' => (x - num as isize, y),
        _ => panic!(),
    }
}

// Was used by part 1, and I thought it was clever
fn _rotate_old(dir: char, rot: char, num: usize) -> char {
    let idx: usize = num / 90;
    let dirs_l = ['N', 'W', 'S', 'E'];
    let dirs_r = ['N', 'E', 'S', 'W'];
    dirs_l.iter().position(|&x| x == dir).unwrap();

    match rot {
        'L' => {
            dirs_l[(dirs_l.iter().position(|&x| x == dir).unwrap() + idx) as usize % dirs_l.len()]
        }
        'R' => {
            dirs_r[(dirs_r.iter().position(|&x| x == dir).unwrap() + idx) as usize % dirs_r.len()]
        }
        _ => panic!(),
    }
}

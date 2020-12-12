use std::fs;
use aoc2020::data::Data;

fn main() {
    let mut old: Data = Data::new(&fs::read_to_string("src/bin/input11.txt").unwrap());
    let mut count: usize = 0;

    let num_filled: usize = loop {
        // Part 1
        // let next: Data = iterate_adj(&old);
        // Part 2
        let next: Data = iterate_diag(&old);
        count += 1 ;
        if old.eq(&next) {
            break num_matching(&next, '#');
        }
        old = next;
    };

    println!("num filled after {} iter: {}", count, num_filled);
}

#[allow(dead_code)]
fn iterate_adj(grid: &Data) -> Data {
    let mut next: Data = grid.clone();

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if should_fill_adj(grid, x, y) {
                next.update(x, y, '#');
            } else if should_empty_adj(grid, x, y) {
                next.update(x, y, 'L');
            } else {
            }
        }
    }

    next
}

fn iterate_diag(grid: &Data) -> Data {
    let mut next: Data = grid.clone();

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if should_fill_diag(grid, x, y) {
                next.update(x, y, '#');
            } else if should_empty_diag(grid, x, y) {
                next.update(x, y, 'L');
            }
        }
    }

    next
}

fn num_matching(grid: &Data, target: char) -> usize {
    let mut count: usize = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if grid.is(x, y, target) {
                count += 1;
            }
        }
    }

    count
}

// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.

// For part 2, if a seat is floor it should progress until it reaches a seat or the end of the map

#[allow(dead_code)]
fn should_fill_adj(grid: &Data, x: usize, y: usize) -> bool {
    grid.is(x, y, 'L') && count_adjacent_matching(grid, x, y, '#') == 0
}

#[allow(dead_code)]
fn should_empty_adj(grid: &Data, x: usize, y: usize) -> bool {
    grid.is(x, y, '#') && count_adjacent_matching(grid, x, y, '#') >= 4
}

#[allow(dead_code)]
fn count_adjacent_matching(grid: &Data, x: usize, y: usize, target: char) -> usize {
    let mut count: usize = 0;

    for &del_x in [-1 as isize, 0, 1].iter() {
        for &del_y in [-1 as isize, 0, 1].iter() {
            if del_x == 0 && del_y == 0 {
                continue;
            }

            let adj_x: isize = x as isize + del_x;
            let adj_y: isize = y as isize + del_y;

            if grid.has_key(adj_x, adj_y) && grid.is(adj_x as usize, adj_y as usize, target) {
                count += 1
            }
        }
    }

    count
}

fn should_fill_diag(grid: &Data, x: usize, y: usize) -> bool {
    grid.is(x, y, 'L') && count_diag_matching(grid, x, y, '#') == 0
}

// Special note this is >= 5 instead of >= 4
fn should_empty_diag(grid: &Data, x: usize, y: usize) -> bool {
    grid.is(x, y, '#') && count_diag_matching(grid, x, y, '#') >= 5
}

fn count_diag_matching(grid: &Data, x: usize, y: usize, target: char) -> usize {
    let mut count: usize = 0;

    for &del_x in [-1 as isize, 0, 1].iter() {
        for &del_y in [-1 as isize, 0, 1].iter() {
            if del_x == 0 && del_y == 0 {
                continue;
            }

            let mut adj_x: isize = x as isize + del_x;
            let mut adj_y: isize = y as isize + del_y;
            while grid.has_key(adj_x, adj_y) && grid.is(adj_x as usize, adj_y as usize, '.') {
                adj_x += del_x;
                adj_y += del_y;
            };

            if grid.has_key(adj_x, adj_y) && grid.is(adj_x as usize, adj_y as usize, target) {
                count += 1
            }
        }
    }

    count
}

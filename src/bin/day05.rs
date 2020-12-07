use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/bin/input05.txt").expect("Could not read file");
    let max_seat = input.lines()
        .map(|l| get_seat_checksum(l))
        .max()
        .unwrap();

    println!("{}", max_seat);

    let mut all_seats: Vec<usize> = input.lines()
        .map(|l| get_seat_checksum(l))
        .collect::<Vec<usize>>();

    all_seats.sort_unstable();

    let mut last: usize = all_seats[0];
    let mut my_seat: usize = 0;

    // println!("{:?}", all_seats);
    for seat in &all_seats[1..] {
        if last != seat - 1 {
            my_seat = seat - 1;
            break;
        }
        else {
            last = *seat;
        }
    };

    println!("my seat {}", my_seat)
}

fn get_seat_checksum(pass: &str) -> usize {
    let row_part = &pass[..7];
    let col_part = &pass[7..];

    let mut row_min = 0;
    let mut row_max = 127;
    for r in row_part.chars() {
        if r == 'B' {
            row_min += ((row_max - row_min) + 1) / 2
        }
        else {
            row_max -= ((row_max - row_min) + 1) / 2
        }
    }
    assert_eq!(row_min, row_max);

    let mut col_min = 0;
    let mut col_max = 7;
    for c in col_part.chars() {
        if c == 'R' {
            col_min += ((col_max - col_min) + 1) / 2
        }
        else {
            col_max -= ((col_max - col_min) + 1) / 2
        }
    }
    assert_eq!(col_min, col_max);

    // println!("row {} col {}", row_min, col_min);

    row_min * 8 + col_min
}
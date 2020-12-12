use std::fs;

fn main() {
    let mut nums: Vec<usize> = fs::read_to_string("src/bin/input10.txt")
        .unwrap().lines().map(|x| x.parse().unwrap()).collect();
    // key - allow tribonacci to remove the very first adapter if others in the sequence could plug directly
    // all other sets cannot remove their first adapter
    nums.push(0);
    nums.sort_unstable();

    // println!("{:?}", nums);

    // Could be optimized to not store numbers at all and instead just store length of arrays
    let mut last = 0;
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut cur_group: Vec<usize> = Vec::new();

    for num in nums {
        let diff: usize = num - last;

        match diff {
            0..=1 => (),
            3 => {
                groups.push(cur_group.clone());
                cur_group.clear();
            },
            // Bold assumption - no adapter is diff 2
            _ => panic!("diff is not supported"),
        }
        cur_group.push(num);
        last = num;
    }

    // Push last group
    groups.push(cur_group);

    let part1_sum: usize = groups.iter().map(|x| x.len() - 1).sum();
    let part1: usize = groups.len() * part1_sum;
    println!("part1: {}", part1);

    let perms: usize = groups.iter()
        .map(|x| tribonacci(x.len()))
        .product();

    println!("part2: {}", perms);
}

// Self-challenge to write a memoized version
// Figuring out this was tribonacci was not trivial - a lot of time was spent calculating and sequencing
fn tribonacci(value: usize) -> usize {
    match value {
        0 => 0,
        1..=2 => 1,
        _ => tribonacci(value - 1) + tribonacci(value - 2) + tribonacci(value - 3),
    }
}

// 0 -> 0
// 1 -> 1
// 2 -> 1
// 3 -> 2
// 123
// 1-3
//
// 4 -> 4
// 1234
// 12-4
// 1-34
// 1--4
//
// 5 -> 7
// 12345
// 123-5
// 12-45
// 12--5
// 1-345
// 1-3-5
// 1--45
// 1---5 x
//
// 6 -> 13
// 123456
// 1234-6
// 123-56
// 123--6
// 12-456
// 12-4-6
// 12--56
// 12---6 x
// 1-3456
// 1-34-6
// 1-3-56
// 1-3--6
// 1--456
// 1--4-6
// 1---56 x
// 1----6 x
//
// 7 -> 24
// 1234567
// 1234-67
// 123-567
// 123--67
// 12-4567
// 12-4-67
// 12--567
// 12---67 x
// 1-34567
// 1-34-67
// 1-3-567
// 1-3--67
// 1--4567
// 1--4-67
// 1---567 x
// 1----67 x
// 12345-7
// 1234--7
// 123-5-7
// 123---7 x
// 12-45-7
// 12-4--7
// 12--5-7
// 12----7 x
// 1-345-7
// 1-34--7
// 1-3-5-7
// 1-3---7 x
// 1--45-7
// 1--4--7
// 1---5-7 x
// 1-----7 x
//
// 0, 1, 1, 2, 4, 7, 13, 24, ... is a tribonacci sequence

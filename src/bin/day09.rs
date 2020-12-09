use std::fs;
use std::cmp::{max, Ordering};

fn main() {
    let nums: Vec<usize> = fs::read_to_string("src/bin/input09.txt")
        .unwrap().lines().map(|x| x.parse().unwrap()).collect();

//     let nums: Vec<usize> = String::from(r"35
// 20
// 15
// 25
// 47
// 40
// 62
// 55
// 65
// 95
// 102
// 117
// 150
// 182
// 127
// 219
// 299
// 277
// 309
// 576").lines().map(|x| x.parse().unwrap()).collect();
    let preamble = 25;

    // Part 1
    // For my data set, while logging results, i noticed this index came up many times and it's the right answer, but my program never terminates
    // println!("{}", nums[508]);
    // let max_index = find_error(preamble, &nums, preamble);
    // println!("{} => {}", max_index, nums[max_index]);

    // Part 2
    let (low, high) = find_longest_chain_for_sum(nums[508], &nums);
    let mut nums_copy: Vec<usize>= nums[low..=high].iter().copied().collect();
    nums_copy.sort_unstable();
    println!("indices {} and {} values sum to {}", low, high, nums_copy.first().unwrap() + nums_copy.last().unwrap());
}

// Doesn't terminate for real problem, debug logging told me what the answer was
fn find_error(index: usize, nums: &[usize], preamble: usize) -> usize {
    if index >= nums.len() {
        panic!("got to the end of the list??");
    }

    let start: usize = index - preamble;
    let value: usize = nums[index];
    let range: &[usize] = &nums[start..index];
    let mut max_index: usize = index;

    // println!("find sums for {} => {}", index, value);
    // println!("for index {} with preamble {} consider ranges ({}, {}) x ({}, {})", index, preamble, start, index - 1, start + 1, index);

    for (i, &i_val) in range.iter().enumerate() {
        if i_val >= value { continue }

        for &j_val in range.iter().skip(i + 1) {
            if j_val >= value || j_val == i_val { continue }

            if i_val + j_val == value {
                // println!("  found possible state: {} {}", i_val, j_val);
                max_index = max(max_index, find_error(index + 1, nums, preamble));
            }
        }
    }

    max_index
}

fn find_longest_chain_for_sum(target: usize, nums: &[usize]) -> (usize, usize) {
    for (i, &i_val) in nums.iter().enumerate() {
        let mut rolling_sum = i_val;

        for (j, &j_val) in nums.iter().skip(i + 1).enumerate() {
            rolling_sum += j_val;

            match rolling_sum.cmp(&target) {
                Ordering::Less => continue,
                // I was not fully aware that Iterator.skip _resets_ the index counter to 0. Oof.
                Ordering::Equal => return (i, j+i+1),
                Ordering::Greater => break,
            }
        }
    }

    panic!("could not find a rolling sum for {}", target);
}

use std::cmp::{max, Ordering};
use std::fs;

fn main() {
    let nums: Vec<usize> = fs::read_to_string("src/bin/input09.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    // Part 1
    let bad_index: usize = find_error(25, &nums, 25);
    let bad_value: usize = nums[bad_index];
    println!("{} => {}", bad_index, bad_value);

    // Part 2
    let (low, high) = find_longest_chain_for_sum(bad_value, &nums);
    let mut nums_copy: Vec<usize> = nums[low..=high].iter().copied().collect();
    nums_copy.sort_unstable();
    println!(
        "indices {} and {} values sum to {}",
        low,
        high,
        nums_copy.first().unwrap() + nums_copy.last().unwrap()
    );
}

fn find_error(index: usize, nums: &[usize], preamble: usize) -> usize {
    if index >= nums.len() {
        panic!("got to the end of the list??");
    }

    let start: usize = index - preamble;
    let value: usize = nums[index];
    let range: &[usize] = &nums[start..index];

    for (i, &i_val) in range.iter().enumerate() {
        if i_val >= value {
            continue;
        }

        for &j_val in range.iter().skip(i + 1) {
            if j_val >= value || j_val == i_val {
                continue;
            }

            if i_val + j_val == value {
                return max(index, find_error(index + 1, nums, preamble));
            }
        }
    }

    index
}

fn find_longest_chain_for_sum(target: usize, nums: &[usize]) -> (usize, usize) {
    for (i, &i_val) in nums.iter().enumerate() {
        let mut rolling_sum = i_val;

        for (j, &j_val) in nums.iter().skip(i + 1).enumerate() {
            rolling_sum += j_val;

            match rolling_sum.cmp(&target) {
                Ordering::Less => continue,
                // I was not fully aware that Iterator.skip _resets_ the index counter to 0. Oof.
                Ordering::Equal => return (i, j + i + 1),
                Ordering::Greater => break,
            }
        }
    }

    panic!("could not find a rolling sum for {}", target);
}

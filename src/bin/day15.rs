use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("src/bin/input15.txt").unwrap();
    let mut counter: usize = 1;
    let mut last: usize = 0;
    let mut spoken_map: HashMap<usize, usize> = HashMap::new();
    let mut value_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for preamb in input.trim_end().split(",") {
        let val: usize = preamb.parse::<usize>().unwrap();
        let pre_vec = value_map.entry(val).or_insert(Vec::new());

        pre_vec.insert(0, counter);
        spoken_map.insert(counter, val);

        last = val;
        counter += 1;
    }

    while counter <= 30000000 {
        // Use last value spoken to derive new spoken value
        let last_vec = value_map.get(&last).unwrap().clone();
        let spoken = if last_vec.len() < 2 {
            0
        } else {
            last_vec[0] - last_vec[1]
        };

        // Updates maps based on now-spoken value (spoken entry has new counter, counter has single spoken)
        let spoken_vec = value_map.entry(spoken).or_insert(Vec::new());
        spoken_vec.insert(0, counter);
        if spoken_vec.len() > 2 {
            spoken_vec.pop();
        }
        spoken_map.insert(counter, spoken);

        last = spoken;
        counter += 1;
    }

    println!("{:?}", spoken_map.get( &2020));
    println!("{:?}", spoken_map.get( &30000000))
}

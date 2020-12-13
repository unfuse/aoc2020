use std::cmp::Ordering;
use std::fs;

// Part 1
#[derive(Debug, Eq, Clone)]
struct BusSchedule {
    id: usize,
    iters: usize,
    next_avail: usize,
}

impl PartialOrd for BusSchedule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BusSchedule {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next_avail.cmp(&other.next_avail)
    }
}

impl PartialEq for BusSchedule {
    fn eq(&self, other: &Self) -> bool {
        self.next_avail.eq(&other.next_avail)
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input13.txt").unwrap();
    let mut data = input.lines();

    let est_wait: usize = data.next().unwrap().parse().unwrap();
    let busses: Vec<String> = data
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.to_string())
        .collect();

    // Part 1
    let soonest_bus: BusSchedule = busses
        .iter()
        .filter(|x| x.as_str().ne("x"))
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| {
            let iters: usize = (est_wait as f64 / x as f64).ceil() as usize;
            BusSchedule {
                id: x,
                iters,
                next_avail: iters * x,
            }
        })
        .min()
        .unwrap();

    println!(
        "soonest bus' wait time is {}",
        soonest_bus.id * (soonest_bus.next_avail - est_wait)
    );

    // Part 2
    let bus_iters: Vec<(usize, usize)> = busses
        .iter()
        .enumerate()
        .filter(|x| x.1.as_str().ne("x"))
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();

    let mut value = 0;
    let mut step = bus_iters.first().unwrap().1;

    for (idx, bus) in bus_iters.iter().skip(1) {
        while (value + idx) % bus != 0 {
            value += step;
        }
        step *= bus;
    }

    println!("first magical timestamp is {}", value)
}

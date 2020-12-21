use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Eq)]
struct TicketRule {
    name: String,
    first_low: usize,
    first_high: usize,
    sec_low: usize,
    sec_high: usize,
}

impl TicketRule {
    fn complies(&self, value: usize) -> bool {
        (self.first_low <= value && value <= self.first_high)
            || (self.sec_low <= value && value <= self.sec_high)
    }
}

#[derive(Debug)]
struct TicketRuleSet {
    rules: HashSet<TicketRule>,
}

impl TicketRuleSet {
    fn find_non_compliant(&self, ticket: &[usize]) -> Option<usize> {
        for &entry in ticket {
            if !self.rules.iter().any(|r| r.complies(entry)) {
                return Some(entry);
            }
        }
        None
    }
}

#[derive(Debug)]
struct TicketSet {
    tickets: HashSet<Vec<usize>>,
}

impl TicketSet {
    fn all_comply_for(&self, rule: &TicketRule, idx: usize) -> bool {
        self.tickets.iter().all(|ticket| rule.complies(ticket[idx]))
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input16.txt").unwrap();
    //     let input = String::from(
    //         r"class: 0-1 or 4-19
    // row: 0-5 or 8-19
    // seat: 0-13 or 16-19
    //
    // your ticket:
    // 11,12,13
    //
    // nearby tickets:
    // 3,9,18
    // 15,1,5
    // 5,14,9",
    //     );

    let r = Regex::new("^(.+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$+").unwrap();

    let mut splits = input.split("\n\n");

    let ticket_rule_set = TicketRuleSet {
        rules: splits
            .next()
            .unwrap()
            .lines()
            .flat_map(|rule_line| {
                r.captures_iter(rule_line).map(|captures| TicketRule {
                    name: String::from(&captures[1]),
                    first_low: captures[2].parse().unwrap(),
                    first_high: captures[3].parse().unwrap(),
                    sec_low: captures[4].parse().unwrap(),
                    sec_high: captures[5].parse().unwrap(),
                })
            })
            .collect::<HashSet<TicketRule>>(),
    };

    let my_ticket: Vec<usize> = splits
        .next()
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let observed_tickets: Vec<Vec<usize>> = splits
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|x| x.trim().split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    // Part 1
    println!(
        "ticket scanning error rate: {}",
        observed_tickets
            .iter()
            .map(|p| {
                match ticket_rule_set.find_non_compliant(p) {
                    None => 0,
                    Some(x) => x,
                }
            })
            .sum::<usize>()
    );

    let valid_ticket_set: TicketSet = TicketSet {
        tickets: observed_tickets
            .iter()
            .filter(|p| ticket_rule_set.find_non_compliant(p) == None)
            .cloned()
            .collect(),
    };

    let mut rule_order: Vec<TicketRule> = Vec::new();

    let found_order = find_rule_order(
        &valid_ticket_set,
        ticket_rule_set.rules,
        &mut rule_order,
        &mut HashMap::new(),
    );

    assert!(found_order);

    println!("{:?}", rule_order);

    println!("\n\nMy ticket has: ");
    for (idx, rule) in rule_order.iter().enumerate() {
        println!("  {} of {}", rule.name, my_ticket[idx]);
    }

    println!(
        "part 2: {}",
        rule_order
            .iter()
            .enumerate()
            .filter(|(_, rule)| rule.name.starts_with("departure"))
            .map(|(i, _)| my_ticket[i])
            .product::<usize>()
    );
}

fn find_rule_order(
    ticket_set: &TicketSet,
    remaining_rules: HashSet<TicketRule>,
    current_ordering: &mut Vec<TicketRule>,
    memo: &mut HashMap<usize, HashSet<TicketRule>>,
) -> bool {
    // println!("{:?}\n{:?}", remaining_rules, current_ordering);
    if remaining_rules.is_empty() {
        return true;
    }

    // To pass onto next layer
    let mut rem_rule_clone = remaining_rules.clone();
    let cur_idx = current_ordering.len();
    for cur_rule in rem_rule_clone.drain() {
        // println!("  try next: {:?}", cur_rule);

        // Check memory for values at this index
        match memo.entry(cur_idx) {
            Entry::Occupied(mem) => {
                // If mem contains the rule, we've already processed it as bad. Skip it
                if mem.get().contains(&cur_rule) {
                    continue;
                }
            }
            Entry::Vacant(mem) => {
                mem.insert(HashSet::new());
            }
        }

        // Derive validity of rule for current index
        if ticket_set.all_comply_for(&cur_rule, cur_idx) {
            // println!("    success! recurse...");
            let mut next_rem_rule: HashSet<TicketRule> = remaining_rules.clone();
            next_rem_rule.remove(&cur_rule);
            current_ordering.push(cur_rule);
            if find_rule_order(ticket_set, next_rem_rule, current_ordering, memo) {
                return true;
            } else {
                current_ordering.pop();
            }
        } else {
            memo.entry(cur_idx).and_modify(|v| {
                v.insert(cur_rule);
            });
        }
    }
    false
}

// Part 2
//   for each remaining rule r, at rule index i
//     if r at i complies with all passport i
//       assign r to i, recurse {set - r, i + 1}
//       if recurse is None -> try again with r + 1 at i
//       if recurse is Some<thing> -> return something

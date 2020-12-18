use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
    rules: Vec<TicketRule>,
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
    tickets: Vec<Vec<usize>>,
}

impl TicketSet {
    fn all_comply_for(&self, rule: &TicketRule, idx: usize) -> bool {
        self.tickets.iter().all(|ticket| rule.complies(ticket[idx]))
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input16.txt").unwrap();

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
            .collect::<Vec<TicketRule>>(),
    };

    let my_ticket: Vec<usize> = splits
        .next()
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let observed_tickets: Vec<Vec<usize>> = splits
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|x| {
            x.trim_end()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
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

    let rule_order = find_rule_order(&valid_ticket_set, ticket_rule_set.rules, Vec::new()).unwrap();

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
    remaining_rules: Vec<TicketRule>,
    current_ordering: Vec<TicketRule>,
) -> Option<Vec<TicketRule>> {
    // println!("{:?}\n{:?}", remaining_rules, current_ordering);
    if remaining_rules.is_empty() {
        return Some(current_ordering);
    }

    let mut rem_rule_clone = remaining_rules.clone();
    while let Some(cur_rule) = rem_rule_clone.pop() {
        // println!("  try next: {:?}", cur_rule);
        let next_idx = current_ordering.len();
        if ticket_set.all_comply_for(&cur_rule, next_idx) {
            // println!("    success! recurse...");
            let next_rem_rule: Vec<TicketRule> = remaining_rules
                .iter()
                .filter(|x| *x != &cur_rule)
                .cloned()
                .collect();
            let mut rule_order_attempt = current_ordering.clone();
            rule_order_attempt.push(cur_rule);
            if let Some(successful_order) =
                find_rule_order(ticket_set, next_rem_rule, rule_order_attempt)
            {
                return Some(successful_order);
            }
        }
    }
    None
}

// Part 2
//   for each remaining rule r, at rule index i
//     if r at i complies with all passport i
//       assign r to i, recurse {set - r, i + 1}
//       if recurse is None -> try again with r + 1 at i
//       if recurse is Some<thing> -> return something

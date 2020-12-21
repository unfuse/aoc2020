use std::fs;
use std::mem::swap;
use std::str::Chars;

struct EqHelper {
    digit_string: String,
    left: Option<isize>,
    right: Option<isize>,
    opt: Option<char>,
}

impl EqHelper {
    fn resolve(&mut self) {
        if self.digit_string.len() > 0 {
            self.right = Some(self.digit_string.parse().unwrap());
            self.digit_string.clear();
        }

        if self.left == None && self.right != None {
            swap(&mut self.left, &mut self.right);
        }

        if let Some(r) = self.right {
            if let Some(l) = self.left {
                if let Some('+') = self.opt {
                    self.left = Some(l + r);
                    self.right = None;
                } else if let Some('*') = self.opt {
                    self.left = Some(l * r);
                    self.right = None;
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input18.txt").unwrap();

    println!(
        "part1: {}",
        input
            .clone()
            .lines()
            .map(|l| format!("{} ", l))
            .map(|l| parse_equation_part1(&mut l.chars()))
            .map(|r| r.unwrap())
            .sum::<isize>()
    );

    // println!("{:?}", parse_equation(&mut "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 ".chars()));
}

fn parse_equation_part1(equation: &mut Chars) -> Option<isize> {
    let mut eq: EqHelper = EqHelper {
        digit_string: String::new(),
        left: None,
        right: None,
        opt: None,
    };

    while let Some(char) = equation.next() {
        match char {
            '(' => {
                eq.right = parse_equation_part1(equation);
            }
            ')' => {
                eq.resolve();
                return eq.left;
            }
            '+' | '*' => {
                eq.opt = Some(char);
            }
            ' ' => {
                eq.resolve();
            }
            c => {
                if c.to_digit(10).is_some() {
                    eq.digit_string.push(c);
                } else {
                    panic!("parsed token I do not understand: {}", c);
                }
            }
        }
    }
    eq.resolve();
    eq.left
}

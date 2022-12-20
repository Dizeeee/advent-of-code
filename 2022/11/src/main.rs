use num::BigUint;

fn main() {
    let input = include_str!("input.txt");

    part1(input.to_string());
    part2(input.to_string());
}

type Item = BigUint;
// #[derive(Debug)]
// struct Item {
//     value: num::BigUint,
//     power: usize,
// }

// impl Item {
//     fn exp(&mut self) {
//         self.value = self.value.pow(self.power as u32);
//     }

//     fn exp_check(&mut self) -> num::BigUint {
//         self.value.pow(self.power as u32)
//     }
// }

// impl Item {
//     fn new(value: num::BigUint) -> Self {
//         Item { value, power: 0 }
//     }

//     fn new_or_old(value: Option<num::BigUint>) -> Option<Self> {
//         if let Some(value) = value {
//             Some(Item::new(value))
//         } else {
//             None
//         }
//     }
// }

// impl std::ops::Rem for Item {
//     type Output = Self;

//     fn rem(self, rhs: Self) -> Self::Output {
//         if self.power == 0 {
//             Item {
//                 value: self.value % rhs.value,
//                 power: 0,
//             }
//         } else {
//             Item {
//                 value: self.value % rhs.exp_check(),
//                 power: 0,
//             }
//         }
//     }
// }

type ItemOrOld = Option<Item>;
type DestMonkey = usize;

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Op,
    test: DestMonkey,
    if_true: DestMonkey,
    if_false: DestMonkey,
    inspections: usize,
}

impl Monkey {
    fn new(input: String) -> Self {
        let mut lines = input.lines();
        lines.next();

        let items = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(2)
            .map(|x| x.trim_end_matches(",").parse().unwrap())
            .collect::<Vec<Item>>();

        let operation_strs = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(3)
            .collect::<Vec<&str>>();

        let operation = match operation_strs[1] {
            "+" => Op::Add(
                operation_strs[0].parse().ok(),
                operation_strs[2].parse().ok(),
            ),
            "*" => Op::Mul(
                operation_strs[0].parse().ok(),
                operation_strs[2].parse().ok(),
            ),
            _ => panic!("Unknown operation"),
        };

        let test: DestMonkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let if_true: DestMonkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let if_false: DestMonkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0,
        }
    }

    fn round(&mut self, worry: bool) -> Vec<(Item, DestMonkey)> {
        let items = self
            .items
            .iter()
            .map(|item| {
                let mut new = self.operation.eval(item.clone());
                if !worry {
                    new /= BigUint::from(3 as u8);
                }

                if &new % num::BigUint::from(self.test) == BigUint::from(0 as u8) {
                    (new, self.if_true)
                } else {
                    (new, self.if_false)
                }
            })
            .collect();

        self.inspections += self.items.len();
        self.items.clear();
        items
    }
}

#[derive(Debug)]
enum Op {
    Add(ItemOrOld, ItemOrOld),
    Mul(ItemOrOld, ItemOrOld),
}

impl Op {
    fn eval(&self, old: Item) -> Item {
        match self {
            Op::Add(a, b) => {
                let a = match a {
                    Some(x) => &*x,
                    None => &old,
                };
                let b = match b {
                    Some(x) => &*x,
                    None => &old,
                };
                a + b
            }
            Op::Mul(a, b) => {
                let a = match a {
                    Some(x) => &*x,
                    None => &old,
                };
                let b = match b {
                    Some(x) => &*x,
                    None => &old,
                };
                a * b
            }
        }
    }
}

fn part1(input: String) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for text in input.split("\n\n") {
        monkeys.push(Monkey::new(text.to_string()));
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let results = monkeys[i].round(false);
            for result in results {
                monkeys[result.1].items.push(result.0.clone());
            }
        }
    }

    // multiply the number of inspections of the two highest monkeys
    let mut inspections = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.reverse();

    println!("Part 1: {}", inspections[0] * inspections[1]);
}

fn part2(input: String) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for text in input.split("\n\n") {
        monkeys.push(Monkey::new(text.to_string()));
    }

    for i in 0..10000 {
        if i % 1 == 0 {
            println!("Round: {}", i);
        }
        for i in 0..monkeys.len() {
            let results = monkeys[i].round(true);
            for result in results {
                monkeys[result.1].items.push(result.0.clone());
            }
        }
    }

    // multiply the number of inspections of the two highest monkeys
    let mut inspections = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.reverse();

    println!("Part 2: {}", inspections[0] * inspections[1]);
}

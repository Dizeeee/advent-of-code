use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let elves: Vec<Elf> = input
        .split("\n\n")
        .map(|s| Elf::new(String::from(s)))
        .collect();

    // get largest sum from all elves
    let mut sum: Vec<u64> = elves.iter().map(|e| e.sum()).collect();
    sum.sort();
    sum.reverse();
    let top_three = sum.iter().take(3);

    // println!("Top three sums:");
    // for i in top_three {
    //     println!("{}", i);
    // }

    println!("Total: {}", &top_three.sum::<u64>());
}

#[derive(Debug)]
struct Elf(Vec<u64>);

impl Elf {
    fn new(input: String) -> Self {
        let elf: Vec<u64> = input
            .split("\n")
            .map(|s| {
                let string = String::from(s);
                string.parse().unwrap()
            })
            .collect();
        Elf(elf)
    }

    fn sum(&self) -> u64 {
        self.0.iter().sum()
    }
}

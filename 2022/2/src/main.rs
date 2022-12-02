fn main() {
    let input: String = String::from(include_str!("input.txt"));
    let lines = input.split("\n");
    let mut score = 0;
    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        match split.last() {
            Some(&"X") => match split.first() {
                Some(&"A") => score += 3,
                Some(&"B") => score += 1,
                Some(&"C") => score += 2,
                _ => (),
            },
            Some(&"Y") => match split.first() {
                Some(&"A") => score += 4,
                Some(&"B") => score += 5,
                Some(&"C") => score += 6,
                _ => (),
            },
            Some(&"Z") => match split.first() {
                Some(&"A") => score += 8,
                Some(&"B") => score += 9,
                Some(&"C") => score += 7,
                _ => (),
            },
            _ => (),
        }
        println!("Score: {}", score);
    }
    println!("{}", score);
}

fn main() {
    let input: String = String::from(include_str!("input.txt"));
    let lines = input.split("\n");
    let mut score = 0;
    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        match split.last() {
            Some(&"X") => {
                score += 1;
                match split.first() {
                    Some(&"A") => score += 3,
                    Some(&"B") => score += 0,
                    Some(&"C") => score += 6,
                    _ => (),
                }
            }
            Some(&"Y") => {
                score += 2;
                match split.first() {
                    Some(&"A") => score += 6,
                    Some(&"B") => score += 3,
                    Some(&"C") => score += 0,
                    _ => (),
                }
            }
            Some(&"Z") => {
                score += 3;
                match split.first() {
                    Some(&"A") => score += 0,
                    Some(&"B") => score += 6,
                    Some(&"C") => score += 3,
                    _ => (),
                }
            }
            _ => (),
        }
        println!("Score: {}", score);
    }
    println!("{}", score);
}

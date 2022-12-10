fn main() {
    let input = include_str!("input.txt");

    part1(input.to_string());
    part2(input.to_string());
}

fn part1(input: String) {
    let mut register: isize = 1;
    let mut cycles: usize = 0;
    let mut strengths = Vec::<isize>::new();

    for instruction in input.lines() {
        if instruction == "noop" {
            cycles += 1;
            if cycles % 40 == 20 {
                strengths.push(register * cycles as isize);
            }
            continue;
        } else {
            let add = instruction.split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<isize>()
                .unwrap();
            for _ in 0..2 {
                cycles += 1;
                if cycles % 40 == 20 {
                    strengths.push(register * cycles as isize);
                }
            }
            register += add;
        }
    }

    println!("Part 1: {}", strengths.iter().sum::<isize>());
}

fn part2(input: String) {
    let mut register: isize = 1;
    let mut cycles: usize = 0;
    let mut drawing: Drawing = [[false; 40]; 6];

    for instruction in input.lines() {
        if instruction == "noop" {
            cycle(Op::Noop, &mut register, &mut cycles, &mut drawing);
            continue;
        } else {
            let add = instruction.split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<isize>()
                .unwrap();
            cycle(Op::AddxFirst(add), &mut register, &mut cycles, &mut drawing);
        }
    }

    println!("Part 2:");
    drawing.map(|row| {
        println!(
            "{}",
            row.map(|val| if val { '#' } else { '.' })
                .iter()
                .collect::<String>()
        );
    });
}

type Drawing = [[bool; 40]; 6];

enum Op {
    Noop,
    AddxFirst(isize),
    AddxSecond(isize),
}

fn cycle(op: Op, register: &mut isize, cycles: &mut usize, drawing: &mut Drawing) {
    match op {
        Op::Noop => {
            draw(drawing, register, cycles);
            *cycles += 1;
        }
        Op::AddxFirst(x) => {
            draw(drawing, register, cycles);
            *cycles += 1;
            cycle(Op::AddxSecond(x), register, cycles, drawing);
        }
        Op::AddxSecond(x) => {
            draw(drawing, register, cycles);
            *cycles += 1;
            *register += x;
        }
    }
}

fn draw(drawing: &mut Drawing, register: &isize, cycles: &mut usize) {
    let row = (*cycles / 40) as usize;
    let col = (*cycles % 40) as usize;
    let pixel = *register - col as isize;
    drawing[row][col] = (-1..=1).contains(&pixel);
}

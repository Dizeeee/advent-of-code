use pathfinding::prelude::{astar, Matrix};

fn main() {
    let input = include_str!("input.txt");

    part1(input.to_string());
}

struct Pos {
    x: usize,
    y: usize,
    successors: Vec<(usize, usize)>,
}

fn part1(input: String) {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    // find S in map
    let mut start = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'S' {
                start = (x, y);
            }
        }
    }

    // find E in map
    let mut end = Pos {
        x: 0,
        y: 0,
        successors: Vec::new(),
    };
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'E' {
                end = (x, y);
            }
        }
    }

    // convert map into Vec<Pos>
    let mut positions: Vec<Pos> = Vec::new();

    map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, num)| {
            let mut successors: Vec<(usize, usize)> = Vec::new();

            if (num - 1) <= map[y - 1][x] || map[y - 1][x] <= (num + 1) {
                successors.push((x, y - 1));
            }
        });
    });
}

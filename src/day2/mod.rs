use itertools::Itertools;

pub fn both() {
    star1();
    star2();
}

fn input() -> std::vec::Vec<((i32, i32), i32)> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .lines()
        .map(|x| x.split(' ').next_tuple().unwrap())
        .map(|(a, b)| {
            (
                (match a {
                    "forward" => (1, 0),
                    "up" => (0, -1),
                    _ => (0, 1),
                }),
                b.parse().unwrap(),
            )
        })
        .collect()
}

pub fn star1() {
    println!("Day 2 - Star 1");
    let result = input()
        .iter()
        .fold((0, 0), |acc, ((h, v), d)| (acc.0 + d * h, acc.1 + d * v));
    println!("Result: {}", result.0 * result.1);
}

pub fn star2() {
    println!("Day 2 - Star 2");
    let result = input().iter().fold((0, 0, 0), |acc, ((f, v), d)| {
        (acc.0 + d * f, acc.1 + d * f * acc.2, acc.2 + v * d)
    });
    println!("Result: {}", result.0 * result.1);
}

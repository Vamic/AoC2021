use itertools::Itertools;

pub fn both() {
    star1();
    star2();
}

fn input() -> std::vec::Vec<i32> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input.lines().map(str::parse).map(Result::unwrap).collect()
}

pub fn star1() {
    println!("Day 1 - Star 1");
    let result = input()
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 1 - Star 2");
    let result = input()
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    println!("Result: {}", result);
}

use itertools::Itertools;

pub fn both() {
    star1();
    star2();
}

fn input() -> Vec<usize> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn star1() {
    println!("Day 7 - Star 1");
    let input = input();
    let target_position = input.iter().sorted().skip((input.len() as f32 / 2 as f32).round() as usize).next().unwrap();
    let result = input.iter().map(|x| (*x as i32 - *target_position as i32).abs()).fold(0, |acc, x| acc + x);
    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 7 - Star 2");
    let input = input();
    let target_position = (input.iter().sum::<usize>() as f32 / input.len() as f32).floor();
    let result = input.iter().map(|x| (*x as i32 - target_position as i32).abs()).fold(0, |acc, x| acc + triangle_number(x));
    println!("Result: {}", result);
}

fn triangle_number(n: i32) -> i32 {
    match n {
        1 => n,
        _ => n + triangle_number(n - 1)
    }
}
use itertools::Itertools;
use std::collections::HashMap;

pub fn both() {
    star1();
    star2();
}

fn input() -> Vec<(Vec<String>, Vec<String>)> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .lines()
        .map(|x| x.split_once('|').unwrap())
        .map(|(a, b)| {
            (
                a.split_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect(),
                b.split_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect()
}

pub fn star1() {
    println!("Day 8 - Star 1");
    let input = input();
    let lengths = [2, 3, 4, 7];
    let result: usize = input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(String::len)
                .filter(|x| lengths.contains(x))
                .count()
        })
        .sum();
    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 8 - Star 2");
    let input = input();
    let result = input.iter().fold(0, |acc, (inp, out)| {
        let one = inp.iter().filter(|x| x.len() == 2).next().unwrap();
        let four = inp.iter().filter(|x| x.len() == 4).next().unwrap();
        acc + out
            .iter()
            .fold(String::new(), |acc, x| match x.len() {
                2 => acc + "1",
                3 => acc + "7",
                4 => acc + "4",
                5 => {
                    if one.chars().all(|c| x.chars().contains(&c)) {
                        acc + "3"
                    } else if four.chars().filter(|c| x.chars().contains(&c)).count() == 3 {
                        acc + "5"
                    } else {
                        acc + "2"
                    }
                }
                6 => {
                    if !one.chars().all(|c| x.chars().contains(&c)) {
                        acc + "6"
                    } else if four.chars().all(|c| x.chars().contains(&c)) {
                        acc + "9"
                    } else {
                        acc + "0"
                    }
                }
                _ => acc + "8",
            })
            .parse::<usize>()
            .unwrap()
    });
    println!("Result: {}", result);
}

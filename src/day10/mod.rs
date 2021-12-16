use itertools::Itertools;
use std::collections::HashMap;

pub fn both() {
    star1();
    star2();
}

fn input() -> Vec<String> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn star1() {
    println!("Day 10 - Star 1");
    let input = input();
    let chunkies = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut bzzt = vec![];

    for line in input {
        let mut seen = vec![];
        for c in line.chars() {
            if chunkies.contains_key(&c) {
                seen.push(chunkies[&c]);
                continue;
            }
            let last = seen.pop();
            if last != None && last.unwrap() == c {
                continue;
            }
            bzzt.push(c);
        }
    }

    let result: usize = bzzt
        .into_iter()
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("how"),
        })
        .sum();

    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 10 - Star 2");
    let input = input();
    let chunkies = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut wao = vec![];

    for line in input {
        let mut seen = vec![];
        let mut approved = true;
        for c in line.chars() {
            if chunkies.contains_key(&c) {
                seen.push(chunkies[&c]);
                continue;
            }
            let last = seen.pop();
            if last != None && last.unwrap() == c {
                continue;
            }
            approved = false;
        }
        if approved {
            wao.push(seen.into_iter().rev());
        }
    }

    let middle = wao.len() / 2;
    let result: usize = wao
        .into_iter()
        .map(|x| {
            x.into_iter().fold(0, |acc, y| match y {
                ')' => acc * 5 + 1,
                ']' => acc * 5 + 2,
                '}' => acc * 5 + 3,
                '>' => acc * 5 + 4,
                _ => panic!("how"),
            })
        })
        .sorted()
        .skip(middle)
        .next()
        .unwrap();
    println!("Result: {}", result);
}

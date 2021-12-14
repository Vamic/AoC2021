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
    let mut result = 0;
    for (inp, out) in input {
        let mut map: [usize; 10] = [0; 10];
        map[1] = inp.iter().position(|x| x.len() == 2).unwrap();
        map[4] = inp.iter().position(|x| x.len() == 4).unwrap();
        for i in 0..inp.len() {
            if map.contains(&i) {
                continue;
            }
            match inp[i].len() {
                3 => map[7] = i,
                5 => {
                    let contains_one = inp[map[1]].chars().all(|c| inp[i].chars().contains(&c));
                    if contains_one {
                        map[3] = i;
                        continue;
                    }
                    let shares_four = inp[map[4]]
                        .chars()
                        .filter(|c| inp[i].chars().contains(&c))
                        .count();
                    if shares_four == 3 {
                        map[5] = i;
                        continue;
                    }
                    map[2] = i;
                }
                6 => {
                    let contains_one = inp[map[1]].chars().all(|c| inp[i].chars().contains(&c));
                    if !contains_one {
                        map[6] = i;
                        continue;
                    }
                    let contains_four = inp[map[4]].chars().all(|c| inp[i].chars().contains(&c));
                    if contains_four {
                        map[9] = i;
                        continue;
                    }
                    map[0] = i;
                }
                _ => map[8] = i,
            }
        }

        result += out
            .into_iter()
            .map(|x| {
                map.iter()
                    .position(|y| {
                        inp[*y].len() == x.len() && inp[*y].chars().all(|c| x.chars().contains(&c))
                    })
                    .unwrap()
            })
            .join("")
            .parse::<usize>()
            .unwrap();
    }
    println!("Result: {}", result);
}

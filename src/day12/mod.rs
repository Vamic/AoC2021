use itertools::Itertools;
use std::collections::VecDeque;

pub fn both() {
    star1();
    star2();
}

fn input() -> Vec<(String, String)> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .lines()
        .map(|x| x.split_once('-').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

pub fn star1() {
    println!("Day 12 - Star 1");
    let input = input();
    let start = String::from("start");
    let mut ongoing_paths = vec![vec![&start]];
    let mut result = 0;
    loop {
        if ongoing_paths.len() == 0 {
            break;
        }
        let last = ongoing_paths[0].last().unwrap();
        let mut options: VecDeque<_> = input
            .iter()
            .filter_map(|(a, b)| {
                if &a == last {
                    Some(b)
                } else if &b == last {
                    Some(a)
                } else {
                    None
                }
            })
            .filter(|x| x != &&start)
            .filter(|x| !is_lower(x) || !ongoing_paths[0].contains(x))
            .collect();
        if options.len() == 0 || last == &&"end" {
            if last == &&"end" {
                result += 1;
            }
            ongoing_paths.remove(0);
            continue;
        }
        let first = options.pop_front().unwrap();
        for option in options {
            let new_path = [ongoing_paths[0].to_vec(), vec![option]].concat();
            ongoing_paths.push(new_path);
        }
        ongoing_paths[0].push(first);
    }

    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 12 - Star 2");
    let input = input();
    let start = String::from("start");
    let mut ongoing_paths = vec![vec![&start]];
    let mut result = 0;
    loop {
        if ongoing_paths.len() == 0 {
            break;
        }
        let last = ongoing_paths[0].last().unwrap();
        let has_dupe = ongoing_paths[0].iter().filter(|x| is_lower(x)).count()
            > ongoing_paths[0]
                .iter()
                .filter(|x| is_lower(x))
                .unique()
                .count();
        let mut options: VecDeque<_> = input
            .iter()
            .filter_map(|(a, b)| {
                if &a == last {
                    Some(b)
                } else if &b == last {
                    Some(a)
                } else {
                    None
                }
            })
            .filter(|x| x != &&start)
            .filter(|x| {
                !is_lower(x)
                    || if !has_dupe {
                        ongoing_paths[0].iter().filter(|y| y == &x).count() < 2
                    } else {
                        !ongoing_paths[0].contains(x)
                    }
            })
            .collect();
        if options.len() == 0 || last == &&"end" {
            if last == &&"end" {
                result += 1;
            }
            ongoing_paths.remove(0);
            continue;
        }
        let first = options.pop_front().unwrap();
        for option in options {
            let new_path = [ongoing_paths[0].to_vec(), vec![option]].concat();
            ongoing_paths.push(new_path);
        }
        ongoing_paths[0].push(first);
    }

    println!("Result: {}", result);
}

fn is_lower(input: &String) -> bool {
    input.chars().all(|c| matches!(c, 'a'..='z'))
}

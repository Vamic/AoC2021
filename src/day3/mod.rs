use itertools::Itertools;
use std::cmp;

fn input() -> std::vec::Vec<Vec<char>> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input.lines().map(|x| x.chars().collect()).collect()
}

pub fn star1() {
    println!("Day 3 - Star 1");
    let input = input();
    let len = input.len();
    let len2 = input[0].len();
    let mut counts: Vec<i32> = vec![(len / 2).try_into().unwrap(); len2];
    for i in 0..len {
        for j in 0..input[i].len() {
            if input[i][j] == '0' {
                counts[j] -= 1;
            }
        }
    }

    let gamma = counts.iter().map(|x| cmp::max(0, cmp::min(1, *x))).join("");
    let epsilon = gamma.replace("0", "_").replace("1", "0").replace("_", "1");

    println!(
        "Result: {}",
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
    );
}

pub fn star2() {
    println!("Day 3 - Star 2");
    let input = input();
    let len2 = input[0].len();
    let mut o2string = "".to_string();
    let mut co2string = "".to_string();
    for j in 0..len2 {
        let mut count = 0;
        let lines: Vec<_> = input.iter().map(|x| x.iter().collect::<String>()).filter(|x| o2string == "" || x.starts_with(&o2string)).collect();
        if lines.len() == 1 {
            o2string = lines.first().unwrap().to_string();
            break;
        }
        for line in lines {
            count += match line.chars().nth(j).unwrap() {
                '0' => -1,
                _ => 1,
            };
        }
        if count >= 0 {
            o2string += "1";
        } else {
            o2string += "0";
        }
    }
    for j in 0..len2 {
        let mut count = 0;
        let lines: Vec<_> = input.iter().map(|x| x.iter().collect::<String>()).filter(|x| co2string == "" || x.starts_with(&co2string)).collect();
        let _len = lines.len();
        if lines.len() == 1 {
            co2string = lines.first().unwrap().to_string();
            break;
        }
        for line in lines {
            count += match line.chars().nth(j).unwrap() {
                '0' => -1,
                _ => 1,
            };
        }
        if count < 0 {
            co2string += "1";
        } else {
            co2string += "0";
        }
    }
    println!("Result: {}", isize::from_str_radix(&o2string, 2).unwrap() * isize::from_str_radix(&co2string, 2).unwrap());
}

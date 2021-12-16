use itertools::Itertools;

pub fn both() {
    star1();
    star2();
}

fn input() -> Vec<Vec<usize>> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .lines()
        .map(|x| {
            x.split_terminator("")
                .skip(1)
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

pub fn star1() {
    println!("Day 9 - Star 1");
    let input = input();
    let mut result = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let current = input[y][x];
            if ![
                (y + 1, x),
                (y.wrapping_sub(1), x),
                (y, x + 1),
                (y, x.wrapping_sub(1)),
            ]
            .into_iter()
            .any(|(y2, x2)| y2 < input.len() && x2 < input[y2].len() && input[y2][x2] <= current)
            {
                result += current + 1;
            }
        }
    }
    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 9 - Star 2");
    let input = input();
    let mut basins: Vec<Vec<(usize, usize)>> = vec![];

    let mut eligible: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .map(|(y, a)| {
            a.iter()
                .enumerate()
                .filter(|(_, b)| **b < 9)
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .collect();

    let first = eligible.drain(0..1).collect::<Vec<(usize, usize)>>()[0];
    let mut basin = vec![first];
    let mut found_last = vec![first];
    loop {
        loop {
            let (found, rest): (Vec<_>, Vec<_>) = eligible.into_iter().partition(|(x, y)| {
                [
                    (*x, y + 1),
                    (*x, y.wrapping_sub(1)),
                    (x + 1, *y),
                    (x.wrapping_sub(1), *y),
                ]
                .iter()
                .any(|a| found_last.contains(&a))
            });
            eligible = rest;
            if found.len() == 0 {
                break;
            }
            basin = [basin, found.to_vec()].concat();
            found_last = found.to_vec();
        }
        if eligible.len() == 0 {
            break;
        }
        let first = eligible.drain(0..1).collect::<Vec<(usize, usize)>>()[0];
        basins = [basins, vec![basin]].concat();
        basin = vec![first];
        found_last = vec![first];
    }

    let result: usize = basins
        .iter()
        .map(|x| x.len())
        .sorted()
        .rev()
        .take(3)
        .product();
    println!("Result: {}", result);
}

use std::cmp;

fn input() -> Vec<((usize, usize), (usize, usize))> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .lines()
        .map(|x| x.split_once(" -> ").unwrap())
        .map(|(a, b)| (a.split_once(',').unwrap(), b.split_once(',').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                (a.parse().unwrap(), b.parse().unwrap()),
                (c.parse().unwrap(), d.parse().unwrap()),
            )
        })
        .collect()
}

pub fn star1() {
    println!("Day 5 - Star 1");
    let input = input();
    let max_coord = input
        .iter()
        .map(|((x1, y1), (x2, y2))| [x1, y1, x2, y2])
        .flatten()
        .max()
        .unwrap();
    let coord_chart = input.iter().fold(
        vec![vec![0; *max_coord + 1]; *max_coord + 1],
        |mut acc, ((y1, x1), (y2, x2))| {
            if x1 == x2 {
                for y in cmp::min(*y1, *y2)..=cmp::max(*y1, *y2) {
                    acc[*x1][y] += 1;
                }
            } else if y1 == y2 {
                for x in cmp::min(*x1, *x2)..=cmp::max(*x1, *x2) {
                    acc[x][*y1] += 1;
                }
            }
            acc
        },
    );
    //println!("{}", coord_chart.iter().map(|x| x.iter().join("")).join("\n")); //Print the result
    let result = coord_chart.iter().flatten().filter(|x| **x > 1).count();
    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 5 - Star 2");
    let input = input();
    let max_coord = input
        .iter()
        .map(|((x1, y1), (x2, y2))| [x1, y1, x2, y2])
        .flatten()
        .max()
        .unwrap();
    let coord_chart = input.iter().fold(
        vec![vec![0; *max_coord + 1]; *max_coord + 1],
        |mut acc, ((y1, x1), (y2, x2))| {
            if x1 == x2 {
                for y in cmp::min(*y1, *y2)..=cmp::max(*y1, *y2) {
                    acc[*x1][y] += 1;
                }
            } else if y1 == y2 {
                for x in cmp::min(*x1, *x2)..=cmp::max(*x1, *x2) {
                    acc[x][*y1] += 1;
                }
            } else {
                let mut y = *y1 as i32;
                let mut y_dir = (y - *y2 as i32).signum() * -1;
                let mut x = *x1 as i32;
                let mut x_dir = (x - *x2 as i32).signum() * -1;
                if y_dir == x_dir && x1 > x2 {
                    y_dir = -1;
                    x_dir = -1;
                }
                for _ in cmp::min(*x1, *x2)..=cmp::max(*x1, *x2) {
                    acc[x as usize][y as usize] += 1;
                    y += y_dir;
                    x += x_dir;
                }
            }
            acc
        },
    );
    //println!("{}",coord_chart.iter().map(|x| x.iter().map(|y| if *y == 0 {".".to_string()} else {y.to_string()}).join("")).join("\n")); //Print the result
    let result = coord_chart.iter().flatten().filter(|x| **x > 1).count();
    println!("Result: {}", result);
}

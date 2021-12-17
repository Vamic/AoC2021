use itertools::Itertools;

pub fn both() {
    star1();
    star2();
}

fn input() -> (Vec<(usize, usize)>, Vec<(char, usize)>) {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    let (points, folds) = str_input.split_once("\n\n").unwrap();
    (
        points
            .lines()
            .map(|x| x.split_once(',').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect(),
        folds
            .lines()
            .map(|x| x.split(' ').last().unwrap())
            .map(|x| x.split_once('=').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect(),
    )
}

pub fn star1() {
    println!("Day 13 - Star 1");
    let (points, folds) = input();
    let mut len_x = folds
        .iter()
        .filter(|(d, _)| *d == 'x')
        .map(|(_, pos)| pos * 2)
        .max()
        .unwrap()
        + 1;
    let mut len_y = folds
        .iter()
        .filter(|(d, _)| *d == 'y')
        .map(|(_, pos)| pos * 2)
        .max()
        .unwrap()
        + 1;
    let mut paper = vec![vec![0; len_x]; len_y];

    for (x, y) in points {
        paper[y][x] = 1;
    }

    for (dir, pos) in folds {
        match dir {
            'x' => {
                len_x = pos;
                for y in 0..len_y {
                    for x in 0..len_x {
                        paper[y][x] += paper[y][pos * 2 - x];
                    }
                }
            }
            _ => {
                len_y = pos;
                for y in 0..len_y {
                    for x in 0..len_x {
                        paper[y][x] += paper[pos * 2 - y][x];
                    }
                }
            }
        }
        break;
    }

    let result = paper
        .iter()
        .take(len_y)
        .map(|x| x.iter().take(len_x))
        .flatten()
        .filter(|x| **x > 0)
        .count();
    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 13 - Star 2");
    let (points, folds) = input();
    let mut len_x = folds
        .iter()
        .filter(|(d, _)| *d == 'x')
        .map(|(_, pos)| pos * 2)
        .max()
        .unwrap()
        + 1;
    let mut len_y = folds
        .iter()
        .filter(|(d, _)| *d == 'y')
        .map(|(_, pos)| pos * 2)
        .max()
        .unwrap()
        + 1;
    let mut paper = vec![vec![0; len_x]; len_y];

    for (x, y) in points {
        paper[y][x] = 1;
    }

    for (dir, pos) in folds {
        match dir {
            'x' => {
                len_x = pos;
                for y in 0..len_y {
                    for x in 0..len_x {
                        paper[y][x] += paper[y][pos * 2 - x];
                    }
                }
            }
            _ => {
                len_y = pos;
                for y in 0..len_y {
                    for x in 0..len_x {
                        paper[y][x] += paper[pos * 2 - y][x];
                    }
                }
            }
        }
    }

    let result = paper
        .into_iter()
        .take(len_y)
        .map(|x| {
            x.into_iter()
                .take(len_x)
                .map(|x| if x > 0 { '#' } else { ' ' })
                .join("")
        })
        .join("\n");
    println!("Result:\n{}", result);
}

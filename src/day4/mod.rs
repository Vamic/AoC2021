pub fn both() {
    star1();
    star2();
}

fn input() -> (Vec<usize>, Vec<Vec<Vec<(usize, bool)>>>) {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    let split = str_input.split_once("\r\n\r\n").unwrap();
    (
        split
            .0
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect(),
        split
            .1
            .split("\n\n")
            .map(|x| {
                x.lines()
                    .map(|y| {
                        y.split_whitespace()
                            .map(str::parse)
                            .map(Result::unwrap)
                            .map(|z| (z, false))
                            .collect()
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn star1() {
    println!("Day 4 - Star 1");
    let (numbers, mut bingos) = input();

    let mut x_matches = vec![vec![0; bingos[0].len()]; bingos.len()];
    let mut y_matches = vec![vec![0; bingos[0].len()]; bingos.len()];
    for num in numbers {
        for i in 0..bingos.len() {
            for x in 0..bingos[i].len() {
                for y in 0..bingos[i][x].len() {
                    if num == bingos[i][x][y].0 {
                        bingos[i][x][y].1 = true;
                        x_matches[i][x] += 1;
                        y_matches[i][y] += 1;
                        if y_matches[i][y] == 5 || x_matches[i][x] == 5 {
                            let unmarked = bingos[i].iter().fold(0, |a, x| {
                                a + x.iter().filter(|y| !y.1).fold(0, |b, y| b + y.0)
                            });
                            println!("Result: {}", num * unmarked);
                            return;
                        }
                    }
                }
            }
        }
    }
}

pub fn star2() {
    println!("Day 4 - Star 2");
    let (numbers, mut bingos) = input();

    let mut x_matches = vec![vec![0; bingos[0].len()]; bingos.len()];
    let mut y_matches = vec![vec![0; bingos[0].len()]; bingos.len()];
    let mut bingoed_bingos: Vec<usize> = [].to_vec();
    for num in numbers {
        for i in 0..bingos.len() {
            if bingoed_bingos.contains(&i) {
                continue;
            }
            for x in 0..bingos[i].len() {
                for y in 0..bingos[i][x].len() {
                    if num == bingos[i][x][y].0 {
                        bingos[i][x][y].1 = true;
                        x_matches[i][x] += 1;
                        y_matches[i][y] += 1;
                        if y_matches[i][y] == 5 || x_matches[i][x] == 5 {
                            bingoed_bingos.push(i);
                            if bingoed_bingos.len() == bingos.len() {
                                let unmarked = bingos[i].iter().fold(0, |a, x| {
                                    a + x.iter().filter(|y| !y.1).fold(0, |b, y| b + y.0)
                                });
                                println!("Result: {}", num * unmarked);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

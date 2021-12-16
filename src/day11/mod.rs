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

fn adjacent((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y.wrapping_sub(1)),
        (x, y + 1),
        (x, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y + 1),
        (x.wrapping_sub(1), y),
        (x.wrapping_sub(1), y.wrapping_sub(1)),
    ]
}

pub fn star1() {
    println!("Day 11 - Star 1");
    let mut input = input();

    let mut charged = vec![];
    let mut result = 0;

    for _step in 0..100 {
        //Charge...
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    charged.push((x, y));
                }
            }
        }
        //Glow!
        while charged.len() > 0 {
            let dumbo = charged.pop().unwrap();
            for (x, y) in adjacent(dumbo) {
                if y >= input.len() || x >= input[y].len() {
                    continue;
                }
                input[y][x] += 1;
                if input[y][x] == 10 {
                    charged.push((x, y));
                }
            }
        }
        //Zzzzz...
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] > 9  {
                    result += 1;
                    input[y][x] = 0;
                }
            }
        }
    }

    println!("Result: {}", result);
}

pub fn star2() {
    println!("Day 11 - Star 2");
    let mut input = input();

    let mut charged = vec![];
    let mut result = 0;

    loop {
        result += 1;
        //Charge...
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    charged.push((x, y));
                }
            }
        }
        //Glow!
        while charged.len() > 0 {
            let dumbo = charged.pop().unwrap();
            for (x, y) in adjacent(dumbo) {
                if y >= input.len() || x >= input[y].len() {
                    continue;
                }
                input[y][x] += 1;
                if input[y][x] == 10 {
                    charged.push((x, y));
                }
            }
        }

        //Zzzzz...
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] > 9  {
                    input[y][x] = 0;
                }
            }
        }

        if input.iter().flatten().all(|x| x == &0) {
            break;
        }
    }

    println!("Result: {}", result);
}

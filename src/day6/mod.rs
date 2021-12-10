fn input() -> Vec<usize> {
    let bytes = include_bytes!("input.txt");
    let str_input = String::from_utf8_lossy(bytes);
    str_input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn star1() {
    println!("Day 6 - Star 1");
    let mut input = input();
    for _ in 0..80 {
        input = input
            .into_iter()
            .map(|mut feesh| {
                if feesh == 0 {
                    feesh = 7;
                    vec![feesh - 1, 8]
                } else {
                    vec![feesh - 1]
                }
            })
            .flatten()
            .collect();
    }
    println!("Result: {}", input.len());
}

pub fn star2() {
    println!("Day 6 - Star 2");
    let input = input();
    const DAYS: usize = 256;

    let mut birthdays = [0; DAYS + 9];
    for feesh in &input {
        birthdays[*feesh] += 1;
    }
    let population = (0..DAYS).fold(input.len(), |pop, i| {
        birthdays[i + 7] += birthdays[i];
        birthdays[i + 9] += birthdays[i];
        pop + birthdays[i]
    });
    println!("Result: {}", population);
}

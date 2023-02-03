use fractory_game::logic::fractal::BoringFractal as Fractal;
use std::io::{stdin, stdout, Write};

fn input_path() -> Option<Vec<usize>> {
    println!("input a path");
    print!("> ");
    stdout().flush().unwrap();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|word| word.parse::<usize>().ok().filter(|num| *num < 4))
        .collect()
}

fn main() {
    let mut frac = Fractal::load(1, [[0, 0, 0, 0], [0, 1, 1, 1]]);
    loop {
        println!("{:?}", frac);
        if let Some(path) = input_path() {
            let prev = frac.set(path.iter().cloned(), 0);
            if prev == 0 {
                frac.set(path, 1);
            }
        }
    }
}

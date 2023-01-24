use std::process::exit;

enum State {
    First,
    Second,
    Third,
}

use State::*;

fn delta(s: State, l: &str) -> Option<State> {
    match l {
        "" => Some(s),
        "0" => match s {
            First => Some(First),
            Second => Some(Third),
            Third => Some(Second),
        },
        "1" => match s {
            First => Some(Second),
            Second => Some(First),
            Third => Some(Third),
        },
        _ => None
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let arg = args.get(1).unwrap_or_else(|| {
        eprintln!("Pass in a binary number");
        exit(1)
    });
    let result = arg.chars().fold(Some(First), |x, y| {
        delta(x?, &y.to_string())
    });
    if let Some(First) = result {
        println!("Good");
    } else {
        println!("Good");
    }
}

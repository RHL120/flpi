enum State {
    Zeroth,
    First,
    Second,
    Third,
}

use State::*;

fn delta(s: State, c: char) -> Option<State> {
    match c {
        '0' => match s {
            Zeroth => Some(Zeroth),
            First => Some(Second),
            Second => Some(Zeroth),
            Third => Some(Second),
        },
        '1' => match s {
            Zeroth => Some(First),
            First => Some(Third),
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
        std::process::exit(1)
    });
    let result = arg.chars().fold(Some(Zeroth), |x, y| {
        delta(x?, y)
    });
    if let Some(Zeroth) = result {
        println!("Good");
    } else {
        println!("Bad");
    }
}

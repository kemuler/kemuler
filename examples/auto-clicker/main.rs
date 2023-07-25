use kemuler::{prelude::*, simulators::enigo::Enigo};
use std::io::stdin;

fn prompt<T>(message: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    println!("{message}");
    read_line().trim().parse::<T>().unwrap()
}

fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let interval = prompt::<u64>("Interval in milliseconds:");
    let button = {
        println!("Mouse button (left, right, middle):");
        let line = read_line().trim().to_lowercase();
        match &line[..] {
            "left" | "l" => MouseButton::Left,
            "right" | "r" => MouseButton::Right,
            "middle" | "m" => MouseButton::Middle,
            _ => panic!("dumbass, choose only left, right, or middle"),
        }
    };

    let mut enigo = Enigo::new();

    loop {
        button.click().sleep_ms(interval).run_with(&mut enigo);
    }
}

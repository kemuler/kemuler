use std::{io::stdin, thread, time::Duration};

use kemuler::{prelude::*, simulate::Simulator, simulators::Enigo};

fn enigo<E: Event<Enigo>>(e: E) {
    e.run_with(&mut kemuler::simulators::Enigo::new())
}

fn main() {
    let stdin = stdin();
    let interval = {
        println!("Interval in milliseconds:");
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let line = line.trim();
        line.parse::<u64>().unwrap()
    };
    let button = {
        println!("Mouse button (left, right, middle):");
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let line = line.trim();
        let line = line.to_lowercase();
        match &line[..] {
            "left" | "l" => MouseButton::Left,
            "right" | "r" => MouseButton::Right,
            "middle" | "m" => MouseButton::Middle,
            _ => panic!("dumbass, choose only left, right, or middle"),
        }
    };
    loop {
        enigo(button.click());
        thread::sleep(Duration::from_millis(interval));
    }
}

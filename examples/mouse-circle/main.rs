use std::{
    io::stdin,
    thread,
    time::{Duration, Instant},
};

use kemuler::{prelude::*, simulate::Simulator, simulators::Enigo};

fn enigo<E: Event<Enigo>>(e: E) {
    e.run_with(&mut kemuler::simulators::Enigo::new())
}

fn main() {
    let stdin = stdin();
    let radius = {
        println!("Radius:");
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let line = line.trim();
        line.parse::<f64>().unwrap()
    };
    let speed = {
        println!("Speed:");
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let line = line.trim();
        line.parse::<f64>().unwrap()
    };
    let offset_x = radius;
    let offset_y = radius;
    let mut delta_time = 0f64;
    let mut time_elasped = 0f64;
    loop {
        let previous_instant = Instant::now();
        time_elasped += delta_time;

        let x = (time_elasped * speed).cos() * radius;
        let y = (time_elasped * speed).sin() * radius;
        let pos_x = x + offset_x;
        let pos_y = y + offset_y;
        enigo(MousePosition.move_to(pos_x as i32, pos_y as i32));
        delta_time = (Instant::now() - previous_instant).as_secs_f64();
    }
}

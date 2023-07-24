use std::{io::stdin, time::Instant};

use kemuler::{prelude::*, simulators::Enigo};

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
    let radius = prompt::<f64>("Radius in pixels:");
    let speed = prompt::<f64>("Speed:");
    let offset_x = radius;
    let offset_y = radius;

    let mut enigo = Enigo::new();

    let mut delta_time = 0f64;
    let mut time_elasped = 0f64;
    loop {
        let previous_instant = Instant::now();
        time_elasped += delta_time;

        let x = (time_elasped * speed).cos() * radius;
        let y = (time_elasped * speed).sin() * radius;
        let pos_x = x + offset_x;
        let pos_y = y + offset_y;
        MousePosition
            .move_to(pos_x as i32, pos_y as i32)
            .run_with(&mut enigo);

        delta_time = (Instant::now() - previous_instant).as_secs_f64();
    }
}

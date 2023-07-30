use std::time::{Duration, Instant};

use crate::prelude::*;
use crate::{assert_event, simulators::string_event_logger::StringEventLogger as S};

use super::Sleep;

#[test]
fn combinator_then() {
    let mut s = S::new();
    let x = Key::F1.down().then(Key::F2.down()).then(Key::F3.up());
    x.run_with(&mut s);
    assert_event!(s, 0, Key::F1.down());
    assert_event!(s, 1, Key::F2.down());
    assert_event!(s, 2, Key::F3.up());
    assert_eq!(s.data.len(), 3);
}

#[test]
fn combinator_sleep() {
    let mut s = S::new();
    let sleep_amount = Duration::from_millis(100);

    let previous = Instant::now();

    let x = Sleep(sleep_amount);
    x.run_with(&mut s);

    let now = Instant::now();

    let diff = now - previous;
    let deviation = Duration::from_millis(2);
    let min = sleep_amount - deviation;
    let max = sleep_amount + deviation;
    assert!(diff >= min && diff <= max);
    assert_eq!(s.data.len(), 0);
}

#[test]
fn combinator_repeat() {
    let mut s = S::new();
    let x = Key::Home.down().repeat(5);
    x.run_with(&mut s);
    assert_event!(s, 0, Key::Home.down());
    assert_event!(s, 1, Key::Home.down());
    assert_event!(s, 2, Key::Home.down());
    assert_event!(s, 3, Key::Home.down());
    assert_event!(s, 4, Key::Home.down());
    assert_eq!(s.data.len(), 5);
}

#[test]
fn combinator_repeat_complex() {
    let mut s = S::new();
    let x = Key::Home.down().repeat(3).then(Key::Tab.up()).repeat(2);
    x.run_with(&mut s);
    assert_event!(s, 0, Key::Home.down());
    assert_event!(s, 1, Key::Home.down());
    assert_event!(s, 2, Key::Home.down());
    assert_event!(s, 3, Key::Tab.up());
    assert_event!(s, 4, Key::Home.down());
    assert_event!(s, 5, Key::Home.down());
    assert_event!(s, 6, Key::Home.down());
    assert_event!(s, 7, Key::Tab.up());
    assert_eq!(s.data.len(), 8);
}

#[test]
fn combinator_iter_seq() {
    let mut s = S::new();
    let x = [Key::F1, Key::F2, Key::F3, Key::F4, Key::F5]
        .iter()
        .map(|k| k.down())
        .iter_seq();
    x.run_with(&mut s);
    assert_event!(s, 0, Key::F1.down());
    assert_event!(s, 1, Key::F2.down());
    assert_event!(s, 2, Key::F3.down());
    assert_event!(s, 3, Key::F4.down());
    assert_event!(s, 4, Key::F5.down());
    assert_eq!(s.data.len(), 5);
}

#[test]
fn combinator_seq() {
    let mut s = S::new();
    let x = (
        Key::DownArrow.down(),
        MouseButton::Left.up(),
        MousePosition.move_to(25, 10),
    )
        .seq();
    x.run_with(&mut s);
    assert_event!(s, 0, Key::DownArrow.down());
    assert_event!(s, 1, MouseButton::Left.up());
    assert_event!(s, 2, MousePosition.move_to(25, 10));
    assert_eq!(s.data.len(), 3);
}

#[test]
fn combinator_seq_empty() {
    let mut s = S::new();
    let x = ().seq();
    x.run_with(&mut s);
    assert_eq!(s.data.len(), 0);
}

#[test]
fn combinator_during() {
    let mut s = S::new();
    let x = Key::Tab.click().during(Key::Alt.down());
    x.run_with(&mut s);
    assert_event!(s, 0, Key::Alt.down());
    assert_event!(s, 1, Key::Tab.down());
    assert_event!(s, 2, Key::Tab.up());
    assert_event!(s, 3, Key::Alt.up());
    assert_eq!(s.data.len(), 4);
}

#[test]
fn combinator_during_nested() {
    let mut s = S::new();
    let x = Key::Tab
        .click()
        .during(Key::Alt.down())
        .during(Key::F1.up());
    x.run_with(&mut s);
    assert_event!(s, 0, Key::F1.up());
    assert_event!(s, 1, Key::Alt.down());
    assert_event!(s, 2, Key::Tab.down());
    assert_event!(s, 3, Key::Tab.up());
    assert_event!(s, 4, Key::Alt.up());
    assert_event!(s, 5, Key::F1.down());
    assert_eq!(s.data.len(), 6);
}

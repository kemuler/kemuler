use std::time::{Duration, Instant};

use crate::prelude::*;
use crate::{assert_events, string_event_logger::StringEventLogger as S};

use super::Sleep;

#[test]
fn combinator_then() {
    let mut s = S::new();
    let x = Key::F1.down().then(Key::F2.down()).then(Key::F3.up());
    x.run_with(&mut s);
    assert_events!(s, 0, Key::F1.down(), Key::F2.down(), Key::F3.up(),);
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
    let max_acceptatble_deviation = Duration::from_millis(50);
    let deviated = diff - sleep_amount;
    assert!(
        deviated <= max_acceptatble_deviation,
        "diff is {} millis, deviated by {} millis",
        diff.as_millis(),
        deviated.as_millis(),
    );
    assert_eq!(s.data.len(), 0);
}

#[test]
#[ignore = "not implemented"]
fn combinator_spin_sleep() {
    todo!()
}

#[test]
fn combinator_repeat() {
    let mut s = S::new();
    let x = Key::Home.down().repeat(5);
    x.run_with(&mut s);
    assert_events!(
        s,
        0,
        Key::Home.down(),
        Key::Home.down(),
        Key::Home.down(),
        Key::Home.down(),
        Key::Home.down(),
    );
    assert_eq!(s.data.len(), 5);
}

#[test]
fn combinator_repeat_complex() {
    let mut s = S::new();
    let x = Key::Home.down().repeat(3).then(Key::Tab.up()).repeat(2);
    x.run_with(&mut s);
    assert_events!(
        s,
        0,
        Key::Home.down(),
        Key::Home.down(),
        Key::Home.down(),
        Key::Tab.up(),
        Key::Home.down(),
        Key::Home.down(),
        Key::Home.down(),
        Key::Tab.up(),
    );
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
    assert_events!(
        s,
        0,
        Key::F1.down(),
        Key::F2.down(),
        Key::F3.down(),
        Key::F4.down(),
        Key::F5.down(),
    );
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
    assert_events!(
        s,
        0,
        Key::DownArrow.down(),
        MouseButton::Left.up(),
        MousePosition.move_to(25, 10),
    );
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
    assert_events!(
        s,
        0,
        Key::Alt.down(),
        Key::Tab.down(),
        Key::Tab.up(),
        Key::Alt.up(),
    );
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
    assert_events!(
        s,
        0,
        Key::F1.up(),
        Key::Alt.down(),
        Key::Tab.down(),
        Key::Tab.up(),
        Key::Alt.up(),
        Key::F1.down(),
    );
    assert_eq!(s.data.len(), 6);
}

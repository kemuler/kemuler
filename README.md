# `kemuler`

Welcome to `kemuler`!

This crate offers a different kind of high level input simulator framework,
utilizing Rust’s type system to its full potential (or just over-engineered)
instead of the usual `key_down(Key)`.

Why? Combinators!

```rust
// use a simulator your heart desired
// (your heart must also implement one if there's none)
let mut s = SomeSimulator::new();

// tuple supports
(
    Key::Shift.down(),

    Key::O.click() // a click is key down then key up,
    Key::H.down(),
    Key::H.up() // these are same as Key::H.click(),

    Sleep::from_ms(25), // sleep 25 milliseconds

    (
        Key::Y.click(),
    ),
    Key::E.click(),
    {
      println!("hello there.");
      println!("maybe use this unintentional feature to debug or something.");
      println!("`()` does nothing but can run.");
    },
    Key::A.click();
    Key::H.click().repeat(5) // repeat anything before the `.repeat` 5 times,

    Key::Up.down(),
).run_with(&mut s)
// this typed the message: "OH YEAHHHHH"

// Note that `.up()`, `.down()`, and `.click()` is implemented on `Key` it self,
// not from a trait. They return something that the `Combine` trait can work with.
````

Currently there are only a few amount of combinators is present.
If you've got some more useful combinator, please submit an issue on [GitHub](kemuler_repo)!

# Simulators
Simulators that are being maintained by this crate.

## Enigo
Cross-platform input simulator.

Support for [`enigo`](enigo_repo).
This is the og crate that helpd me make this crate.

## Windows
WIP, currently on the branch `windows`.

Windows input simulator.
An improvement over [`enigo`](enigo_repo)'s Windows implementation.

Differences to [`enigo`](enigo_repo):
  Mouse input will work on application (mostly games)
  that uses DirectX/DirectInput/somethingsomething;
  that is currently not the case on [`enigo`](enigo_repo).
  See [issue](https://github.com/enigo-rs/enigo/issues/172).

  `VirtualKey` enum's variants are more easier to search for.
  They had more intuitive names and doc aliases are added.
  More detailed documentation is included.

- [enigo_repo]: https://github.com/enigo-rs/enigo "Enigo Repository"
- [kemuler_repo]: https://github.com/Multirious/kemuler "Kemuler Repository"

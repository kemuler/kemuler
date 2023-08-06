# `kemuler`

*this crate is currently in a rapid development phase.
compilation failure will probably be introduced with every 5 commits lol*

Welcome to `kemuler`!

This crate offers a different kind of high level input simulator framework,
utilizing Rust’s type system to its full potential *(or just over-engineered)*
instead of the usual `key_down(Key)`.

Why? Combinators!

```rust
# // This exmaple is using a private simulator for demonstration purposes.
# // It is not available in public API.
# use kemuler::string_event_logger::StringEventLogger as Simulator;
use kemuler::prelude::*;
// use a simulator your heart desired
// (your heart must also implement one if there's none)
let mut s = Simulator::new();

// tuple supports
(
    Key::Shift.down(),

    Char('o').click(),
    // down and then up is the same as `.click`
    Char('h').down(),
    Char('h').up(),

    Char('y').click(),
    Char('e').click(),
    Char('a').click(),
    // repeat anything before the `.repeat` 5 times,
    Char('h').click().repeat(5),

    Key::Shift.up(),
)
    // `.seq` wraps the tuple with `Sequence` type to mark it as simulatable
    .seq()
    .run_with(&mut s)
// this typed the message: "OH YEAHHHHH"
```

Currently, there are only a few amount of combinators present.
If you've got some more useful combinator, please submit an issue on [GitHub][kemuler_repo]!

See crate's document for more examples.

# Simulators
Simulators that are being maintained by this crate.
(a simulator is just a backend)

- [`kemuler-enigo`](https://github.com/kemuler/kemuler-enigo)
- [`kemuler-windows`](https://github.com/kemuler/kemuler-windows)

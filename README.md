# `kemuler`

Welcome to an input simulation crate, `kemuler`!

This crate offers a different kind of high level input simulator framework,
utilizing Rust’s type system to its full potential (or just over-engineered)
instead of the usual `key_down(Key)`.
Why?
Combinators!

Current features:
- Multiple backends support (called simulator in here).
  Built-ins:
  - Enigo,
  - Windows (WIP),
- Combinators

Some drawbacks:
- Combinator currently can only combine for the same simulator;
  any simulator combinator is currently on a separated branch.
- Only a few amount of combinators is present.
  If you've got some more useful combinator, please submit an issue on `GitHub`!

# Simulators
Simulators that is maintain by this crate.

## Enigo
This is the og crate that helpd me make this crate.
It is cross-platform.

## Windows (WIP)
Currently on branch `windows`.
Simulator Windows

Has some slight improvement from Enigo which is:
  Mouse input will work on application (mostly games)
  that uses DirectX/DirectInput/somethingsomething;
  that is currently not the case on Enigo.

  Better documentation.
  `VirtualKey` enum's variants is more easier to search for.
  They had more intuitive names and doc aliases are added.
  More detailed documentation is included.

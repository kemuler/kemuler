# `kemuler`

Welcome to an experimental input simulation crate, `kemuler`!

This crate offers a different kind of high level input simulator framework,
utilizing Rust’s type system to its full potential (or just over-engineered)
instead of the usual `key_down(Key)`.
Why?
Combinators!

Current features:
- Multiple backends support (called simulator in here).
  Built-ins:
  - Enigo (The crate that helped me make this crate and it is cross-platform.)
  - (WIP, seperated branch) Windows (support for Window's DirectX/DirectInput something something game something something)
- Combinator

Some drawbacks:
- Combinator currently can only combine for the same simulator;
  any simulator combinator is currently on a separated branch.
- Only a few amount of combinators is present.
  If you've got some more useful combinator, please submit an issue on `GitHub`!

# Why is it experimental?
This crate doesn't really solve any problem other than just easier to do
input simulation (maybe that will be the selling point).
It's also fresh out so some breaking changes might occur in the future.
This crate is just me playing around with the language.
We'll see if this crate will actually get any traction.
If it did, then experimental tag will be stripped lol.

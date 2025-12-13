# Rustorio &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs] [![Discord]][discord.gg]

[Latest Version]: https://img.shields.io/crates/v/rustorio.svg
[crates.io]: https://crates.io/crates/rustorio
[Docs]: https://img.shields.io/docsrs/rustorio.svg
[docs.rs]: https://docs.rs/rustorio
[Discord]: https://dcbadge.limes.pink/api/server/uKJugp85Fk?style=flat
[discord.gg]: https://discord.gg/uKJugp85Fk

The first game written _and played_ entirely in Rust's type system. Not just do you play by
writing Rust code, the rules of the game are enforced by the Rust compiler! If
you can write the program so it compiles and doesn't panic, you win!
## How to play
1. Install [Rust](https://www.rust-lang.org/tools/install). Specifically it's
   important to have the entire rustup toolchain and cargo, all of which you get
   automatically by following the instructions in the link.
2. Install `rustorio` by running `cargo +nightly install rustorio`.
3. Set up a new Rustorio project by running `rustorio setup <path>`, where
   `<path>` is the directory you want to create the project in (defaults to
   '.').
4. Under `src/bin/tutorial/` you will find a tutorial save. You can start by
   playing that one.
5. Playing the game consists of filling out the `user_main` function in the
   `main.rs` file in the save game folder created for you.
6. Run with `rustorio play <save name>` (e.g. `rustorio play tutorial`). This will compile and run your save. If
   it compiles and completes without panicking, you win! It'll then tell you how
   many ticks it took you to win.
### After the tutorial
To play other game modes, run `rustorio new-game` and specify a game mode.
Use `rustorio new-game --help` to see all available game modes.
## Rules
The rules are mostly enforced by the compiler. The only two (current) exceptions are:
1. Do not remove `#![forbid(unsafe_code)]` at the top of the `main.rs` file.
2. Do not exploit unsoundness in the compiler.
Both these would allow you to bypass the rules enforced by the compiler and make the game trivial.
If you find other ways to bypass the rules or to do things that feel like cheating (e.g. [this issue](https://github.com/albertsgarde/rustorio/issues/1)),
please file an issue!
Part of my interest in this project is seeing how close we can get to rule out all possible
cheating vectors using only the Rust compiler. So I'd love to hear about any ways to cheat.
## Help
Documentation for the Rustorio library can be found
[here](https://docs.rs/rustorio/latest/rustorio/). A good place to start
is to build a
[furnace](https://docs.rs/rustorio/latest/rustorio//buildings/struct.Furnace.html)
and start
[mining](hhttps://docs.rs/rustorio/latest/rustorio//fn.mine_iron.html) and
smelting iron. Alternatively, you can work backwards by looking at the
[recipe](https://docs.rs/rustorio/latest/rustorio//recipes/struct.PointRecipe.html)
for points to figure out how to get them.
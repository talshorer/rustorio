//! [![github]](https://github.com/albertsgarde/rustorio)&ensp;[![crates-io]](https://crates.io/crates/rustorio)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//!
//! The first game written _and played_ entirely in Rust's type system. Not just do you play by
//! writing Rust code, the rules of the game are enforced by the Rust compiler! If
//! you can write the program so it compiles and doesn't panic, you win!
//!
//! ## How to play
//!
//! 1. Install [Rust](https://www.rust-lang.org/tools/install). Specifically it's
//!    important to have the entire rustup toolchain and cargo, all of which you get
//!    automatically by following the instructions in the link.
//! 2. Install `rustorio` by running `cargo install rustorio`.
//! 3. Set up a new Rustorio project by running `rustorio setup <path>`, where
//!    `<path>` is the directory you want to create the project in (defaults to
//!    '.').
//! 4. Under `src/bin/tutorial/` you will find a tutorial save. You can start by
//!    playing that one.
//! 5. Playing the game consists of filling out the `user_main` function in the
//!    `main.rs` file in the save game folder created for you.
//! 6. Run with `rustorio play <save name>`. This will compile and run your save. If
//!    it compiles and completes without panicking, you win! It'll then tell you how
//!    many ticks it took you to win.
//!
//! ### After the tutorial
//! To play other game modes, run `rustorio new-game` and specify a game mode.
//! Use `rustorio new-game --help` to see all available game modes.
//!
//! ## Rules
//!
//! The rules are enforced by the compiler. Only thing I'd say is to not remove the
//! `#![deny(unsafe_code)]` at the top of the `main.rs` file, as using unsafe code
//! can bypass most of what the compiler enforces. If you think you've found a way
//! to cheat the game that isn't caught by the compiler, please file an issue! Part
//! of my interest in this project is whether I can actually rule out all possible
//! cheating vectors using only the Rust compiler. I haven't found a way to cheat
//! yet, but I'm sure others will be more creative than me!
//!
//! ## Help
//!
//! Documentation for the Rustorio library can be found
//! [here](https://albertsgarde.github.io/rustorio/rustorio). A good place to start
//! is to build a
//! [furnace](https://albertsgarde.github.io/rustorio/rustorio/buildings/struct.Furnace.html)
//! and start
//! [mining](https://albertsgarde.github.io/rustorio/rustorio/fn.mine_iron.html) and
//! smelting iron. Alternatively, you can work backwards by looking at the
//! [recipe](https://albertsgarde.github.io/rustorio/rustorio/recipes/struct.PointRecipe.html)
//! for points to figure out how to get them.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)] // silence the “still incomplete” lint

pub mod buildings;
pub mod gamemodes;
pub mod recipes;
mod resources;
mod tick;

use std::sync::Once;

pub use resources::{Bundle, Resource, ResourceType};
pub use tick::Tick;

use crate::gamemodes::{GameMode, StartingResources};

static ONCE: Once = Once::new();

/// Runs your play. If it is run multiple times, it will panic. This is to prevent using multiple threads to cheat.
pub fn play<G: GameMode>(main: fn(Tick, G::StartingResources) -> (Tick, G::VictoryResources)) -> ! {
    let mut call_once_ran = false;
    ONCE.call_once(|| call_once_ran = true);
    if !call_once_ran {
        panic!("play() can only be called once per program execution to prevent cheating via multithreading.");
    }
    let tick = Tick::start();
    let start_resources = G::StartingResources::init();
    let (tick, _points) = main(tick, start_resources);
    println!("You won in {} ticks!", tick.cur());
    std::process::exit(0);
}

/// A bundle of specifically iron ore.
pub type IronOreBundle<const AMOUNT: u32> = Bundle<{ ResourceType::IronOre }, AMOUNT>;

/// Mines iron ore. Takes 2 ticks to mine 1 ore.
pub fn mine_iron<const AMOUNT: u32>(tick: &mut Tick) -> IronOreBundle<AMOUNT> {
    for _ in 0..(2 * AMOUNT) {
        tick.next();
    }
    Bundle::new()
}

/// A bundle of specifically copper ore.
pub type CopperOreBundle<const AMOUNT: u32> = Bundle<{ ResourceType::CopperOre }, AMOUNT>;

/// Mines copper ore. Takes 2 ticks to mine 1 ore.
pub fn mine_copper<const AMOUNT: u32>(tick: &mut Tick) -> CopperOreBundle<AMOUNT> {
    for _ in 0..(2 * AMOUNT) {
        tick.next();
    }
    Bundle::new()
}

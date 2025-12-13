#![feature(generic_const_exprs)]
#![allow(incomplete_features)] // silence the “still incomplete” lint
//! The core engine for Rustorio.
//! Only relevant if you are writing a mod for Rustorio.
//! To play the game, depend on the `rustorio` crate instead.
//!
//! For more information, see the [repo](https://github.com/albertsgarde/rustorio)

pub mod gamemodes;
pub mod research;
pub mod resources;
pub mod tick;

use std::sync::Once;

use crate::{
    gamemodes::{GameMode, StartingResources},
    tick::Tick,
};

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

pub trait Sealed {}

pub mod mod_reexports {
    pub use crate::{
        gamemodes::GameMode,
        play,
        research::Research,
        resources::{Bundle, InsufficientResourceError, Resource},
        tick::Tick,
    };
}

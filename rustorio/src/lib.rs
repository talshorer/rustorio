//! The base mod for Rustorio.
//! Contains all the main content of the game.
//! Your saves should depend on this crate.
//!
//! For more information, including help on getting started, see the [repo](https://github.com/albertsgarde/rustorio)

pub mod buildings;
pub mod gamemodes;
pub mod guide;
pub mod recipes;
pub mod research;
pub mod resources;

pub use rustorio_engine::mod_reexports::*;
use rustorio_engine::resources::bundle;

use crate::resources::{CopperOre, IronOre};

/// Mines iron ore. Takes 2 ticks to mine 1 ore.
pub fn mine_iron<const AMOUNT: u32>(tick: &mut Tick) -> Bundle<IronOre, AMOUNT> {
    tick.advance_by(2 * AMOUNT as u64);
    bundle()
}

/// Mines copper ore. Takes 2 ticks to mine 1 ore.
pub fn mine_copper<const AMOUNT: u32>(tick: &mut Tick) -> Bundle<CopperOre, AMOUNT> {
    tick.advance_by(2 * AMOUNT as u64);
    bundle()
}

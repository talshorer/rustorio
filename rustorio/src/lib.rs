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
    if ONCE.is_completed() {
        panic!("rustorio::play() can only be called once per program run.");
    }
    ONCE.call_once(|| {});
    let tick = Tick::start();
    let start_resources = G::StartingResources::init();
    let (tick, _points) = main(tick, start_resources);
    println!("You won in {} ticks!", tick.cur());
    std::process::exit(0);
}

pub type IronOreBundle<const AMOUNT: u32> = Bundle<{ ResourceType::IronOre }, AMOUNT>;

/// Mines iron ore. Takes 2 ticks to mine 1 ore.
pub fn mine_iron<const AMOUNT: u32>(tick: &mut Tick) -> IronOreBundle<AMOUNT> {
    for _ in 0..(2 * AMOUNT) {
        tick.next();
    }
    Bundle::new()
}

pub type CopperOreBundle<const AMOUNT: u32> = Bundle<{ ResourceType::CopperOre }, AMOUNT>;

/// Mines copper ore. Takes 2 ticks to mine 1 ore.
pub fn mine_copper<const AMOUNT: u32>(tick: &mut Tick) -> CopperOreBundle<AMOUNT> {
    for _ in 0..(2 * AMOUNT) {
        tick.next();
    }
    Bundle::new()
}

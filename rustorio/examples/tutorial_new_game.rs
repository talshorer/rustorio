#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, ResourceType, Tick,
    gamemodes::{self},
};

type GameMode = gamemodes::Tutorial;

type StartingResources = <GameMode as gamemodes::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<{ ResourceType::Copper }, 1>) {
    let StartingResources { iron } = starting_resources;

    todo!("Use the iron to create a smelter, mine some copper ore, and smelt it into copper ingots to win the game.")
    // For more information on building a furnace, see https://albertsgarde.github.io/rustorio/rustorio/buildings/struct.Furnace.html
    // To mine copper ore, see https://albertsgarde.github.io/rustorio/rustorio/fn.mine_copper.html

    // To win, simply return the `Tick` and a `Bundle` containing 1 copper ingot.
    // To get a `Bundle` containing copper ingots, you can either take it directly from a furnace using `Furnace::take_output`,
    // or create extract it from a `Resource` using `Resource::bundle`.
}

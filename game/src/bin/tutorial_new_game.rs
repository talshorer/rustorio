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
}

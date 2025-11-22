use rustorio::{
    self, Bundle, ResourceType, Tick,
    gamemodes::{self},
};

type GameMode = gamemodes::Standard;

type StartingResources = <GameMode as gamemodes::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<{ ResourceType::Point }, 10>) {
    let StartingResources { iron } = starting_resources;

    todo!("Return the `tick` and the victory resources to win the game!")
}

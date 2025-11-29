use rustorio::{
    self, Bundle, Resource, ResourceType, Tick,
    buildings::Furnace,
    gamemodes::{self},
    recipes::CopperSmelting,
};

type GameMode = gamemodes::Tutorial;

type StartingResources = <GameMode as gamemodes::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<{ ResourceType::Copper }, 1>) {
    let StartingResources { iron } = starting_resources;

    let mut furnace = Furnace::build(&tick, CopperSmelting, iron);

    let mut copper_ore = Resource::empty();

    copper_ore += rustorio::mine_copper::<2>(&mut tick);

    furnace.add_input(&tick, copper_ore.bundle::<2>().unwrap());
    tick.advance_until(|tick| furnace.cur_output(tick) > 0, 100);

    let win_bundle = furnace.take_output_bundle(&tick).unwrap();
    (tick, win_bundle)
}

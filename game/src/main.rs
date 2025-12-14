use rustorio::{
    Bundle, Resource, Tick, buildings::Furnace, gamemodes::Tutorial, recipes::CopperSmelting, resources::Copper,
};

type GameMode = Tutorial;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Copper, 4>) {
    let StartingResources { iron, guide: _ } = starting_resources;

    let mut furnace = Furnace::build(&tick, CopperSmelting, iron);

    let copper_ore = rustorio::mine_copper::<8>(&mut tick);

    furnace.input(&tick).add(copper_ore);
    tick.advance_until(|tick| furnace.output(tick).amount() > 0, 100);

    let win_bundle = furnace.output(&tick).bundle().unwrap();
    (tick, win_bundle)
}

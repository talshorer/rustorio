use rustorio::{
    Bundle, Tick, buildings::Furnace, gamemodes::Tutorial, recipes::CopperSmelting,
    resources::Copper,
};

type GameMode = Tutorial;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

#[test]
fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Copper, 4>) {
    let StartingResources { iron, guide: _ } = starting_resources;

    let mut furnace = Furnace::build(&tick, CopperSmelting, iron);

    let copper_ore = rustorio::mine_copper::<8>(&mut tick);

    furnace.inputs(&tick).0.add(copper_ore);
    tick.advance_until(|tick| furnace.outputs(tick).0.amount() >= 4, 100);

    let win_bundle = furnace.outputs(&tick).0.bundle().unwrap();
    (tick, win_bundle)
}

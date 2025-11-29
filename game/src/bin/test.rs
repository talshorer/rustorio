use rustorio::{
    buildings::{Assembler, Furnace},
    gamemodes,
    recipes::{CopperSmelting, IronSmelting, PointRecipe},
};

type GameMode = gamemodes::Standard;

type StartingResources = <GameMode as gamemodes::GameMode>::StartingResources;

pub fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(
    mut tick: rustorio::Tick,
    starting_resources: StartingResources,
) -> (rustorio::Tick, rustorio::Bundle<{ rustorio::ResourceType::Point }, 10>) {
    tick.log(false);

    let StartingResources { iron } = starting_resources;

    let mut furnace = Furnace::build(&tick, IronSmelting, iron);

    let iron_ore = rustorio::mine_iron::<500>(&mut tick);

    let mut copper_ore = rustorio::Resource::empty();
    furnace.add_input(&tick, iron_ore.to_resource());
    while furnace.cur_input(&tick) > 0 {
        copper_ore += rustorio::mine_copper::<1>(&mut tick);
    }

    println!("Copper ore mined: {}", copper_ore.amount());

    let mut iron = furnace.empty_output(&tick);
    println!("Iron ingots produced: {}", iron.amount());

    let mut furnace = furnace.change_recipe(CopperSmelting).unwrap();

    furnace.add_input(&tick, copper_ore.bundle::<200>().unwrap());
    tick.advance_until(|tick| furnace.cur_input(tick) == 0);

    let mut copper = furnace.empty_output(&tick);
    println!("Copper ingots produced: {}", copper.amount());

    let mut assembler = Assembler::build(&tick, PointRecipe, iron.bundle().unwrap(), copper.bundle().unwrap());
    println!("Iron left: {}", iron.amount());
    println!("Copper left: {}", copper.amount());

    assembler.add_input1(&tick, iron.bundle::<235>().unwrap());
    assembler.add_input2(&tick, copper.bundle::<90>().unwrap());
    tick.advance_until(|tick| assembler.cur_output(tick) >= 10);

    let points = assembler.take_output_bundle(&tick).unwrap();
    (tick, points)
}

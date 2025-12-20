use rustorio::{
    Technology, Tick,
    buildings::{Assembler, Furnace},
    gamemodes::Standard,
    recipes::{CopperSmelting, IronSmelting, RedScienceRecipe},
    resources::Point,
};

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

pub fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, rustorio::Bundle<Point, 10>) {
    tick.log(false);

    let StartingResources {
        iron,
        points_technology,
    } = starting_resources;

    let mut furnace = Furnace::build(&tick, IronSmelting, iron);

    let iron_ore = rustorio::mine_iron::<500>(&mut tick);

    let mut copper_ore = rustorio::Resource::empty();
    furnace.inputs(&tick).0.add(iron_ore.to_resource());
    while furnace.inputs(&tick).0.cur() > 0 {
        copper_ore += rustorio::mine_copper::<1>(&mut tick);
    }

    println!("Copper ore mined: {}", copper_ore.amount());

    let mut iron = furnace.outputs(&tick).0.empty();
    println!("Iron ingots produced: {}", iron.amount());

    let mut furnace = furnace.change_recipe(CopperSmelting).unwrap();

    furnace.inputs(&tick).0.add(copper_ore.bundle::<200>().unwrap());
    tick.advance_until(|tick| furnace.inputs(tick).0.cur() == 0, u64::MAX);

    let mut copper = furnace.outputs(&tick).0.empty();
    println!("Copper ingots produced: {}", copper.amount());

    let mut assembler = Assembler::build(
        &tick,
        RedScienceRecipe,
        iron.bundle().unwrap(),
        copper.bundle().unwrap(),
    );
    println!("Iron left: {}", iron.amount());
    println!("Copper left: {}", copper.amount());

    assembler.inputs(&tick).0.add(iron.bundle::<5>().unwrap());
    assembler.inputs(&tick).1.add(copper.bundle::<5>().unwrap());
    tick.advance_until(|tick| assembler.outputs(&tick).0.cur() >= 5, 100);
    let red_science = assembler.outputs(&tick).0.take_bundle().unwrap();

    let points_recipe = points_technology.research(red_science);
    println!("Points researched!");

    println!("Iron left: {}", iron.amount());
    println!("Copper left: {}", copper.amount());

    let mut assembler = assembler.change_recipe(points_recipe).unwrap();

    assembler.inputs(&tick).0.add(iron);
    assembler.inputs(&tick).1.add(copper);
    tick.advance_until(|tick| assembler.outputs(&tick).0.cur() >= 10, 10000);

    let points = assembler.outputs(&tick).0.take_bundle().unwrap();
    (tick, points)
}

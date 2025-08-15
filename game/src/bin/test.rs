use revolution::{
    buildings::{Assembler, Furnace},
    gamemodes,
    recipes::{CopperSmelting, IronSmelting, PointRecipe},
};

type GameMode = gamemodes::Standard;

type StartingResources = <GameMode as gamemodes::GameMode>::StartingResources;

pub fn main() {
    revolution::play::<GameMode>(user_main);
}

fn user_main(
    mut tick: revolution::Tick,
    starting_resources: StartingResources,
) -> (
    revolution::Tick,
    revolution::Bundle<{ revolution::ResourceType::Point }, 10>,
) {
    tick.log(false);

    let StartingResources { iron } = starting_resources;

    let mut furnace = Furnace::<IronSmelting>::build(&tick, iron);

    let iron_ore = revolution::mine_iron::<500>(&mut tick);

    let mut copper_ore = revolution::Resource::empty();
    furnace.add_input(&tick, iron_ore);
    while furnace.cur_input(&tick) > 0 {
        copper_ore += revolution::mine_copper::<1>(&mut tick);
    }

    println!("Copper ore mined: {}", copper_ore.amount());

    let mut iron = furnace.empty_output(&tick);
    println!("Iron ingots produced: {}", iron.amount());

    let mut furnace = furnace.change_recipe::<CopperSmelting>().unwrap();

    furnace.add_input(&tick, copper_ore.bundle::<200>().unwrap());
    while furnace.cur_input(&tick) > 0 {
        tick.next();
    }

    let mut copper = furnace.empty_output(&tick);
    println!("Copper ingots produced: {}", copper.amount());

    let mut assembler = Assembler::<PointRecipe>::build(&tick, iron.bundle().unwrap(), copper.bundle().unwrap());
    println!("Iron left: {}", iron.amount());
    println!("Copper left: {}", copper.amount());

    assembler.add_input1(&tick, iron.bundle::<235>().unwrap());
    assembler.add_input2(&tick, copper.bundle::<90>().unwrap());
    while assembler.cur_output(&tick) < 10 {
        tick.next();
    }

    let points = assembler.take_output(&tick).unwrap();
    (tick, points)
}

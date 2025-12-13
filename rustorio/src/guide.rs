//! A guide that provides hints to the player during the tutorial.

use std::process;

use rustorio_engine::ResourceType;

use crate::{
    Bundle, Resource, Tick,
    buildings::Furnace,
    recipes::FurnaceRecipe,
    resources::{Copper, CopperOre, Iron, IronOre},
};

/// A topic that the guide can provide hints about.
trait GuideTopic {
    fn hint() -> &'static str;
}

impl GuideTopic for Tick {
    fn hint() -> &'static str {
        "The `Tick` object you are given handles the passage of time in the game. You can use methods like `Tick::advance` or `Tick::advance_until` to make time pass, which is necessary for buildings to process resources. Some functions like `mine_iron` and `mine_copper` take a mutable reference to the Tick to let them advance time internally.
For more information, see https://docs.rs/rustorio/latest/rustorio/struct.Tick.html"
    }
}

impl GuideTopic for Resource<Iron> {
    fn hint() -> &'static str {
        "In this tutorial you start with 10 iron. You can use iron to build buildings like Furnaces and Assemblers.
Try building a Furnace using `Furnace::build`. If you're in doubt about what recipe to pick, try `CopperSmelting` to smelt copper ore into copper ingots."
    }
}

impl<R: FurnaceRecipe> GuideTopic for Furnace<R> {
    fn hint() -> &'static str {
        "Congratulations on building your first Furnace! If you haven't already, mine some copper ore using `mine_copper`. You can add the ore to the furnace using `Furnace::add_input`. If you then use `Tick::advance` to make ticks pass, the ore will turn into ingots which can be extracted using `Furnace::take_output`."
    }
}

impl GuideTopic for Resource<CopperOre> {
    fn hint() -> &'static str {
        "Great job on mining some copper ore! Add the ore to a Furnace using `Furnace::add_input`, then advance time using `Tick::advance` to smelt the ore into copper ingots. Finally, extract the ingots using `Furnace::take_output`.

If you don't have a Furnace yet, build one using `Furnace::build`, and use the `CopperSmelting` recipe to smelt copper ore into copper ingots."
    }
}

impl GuideTopic for Resource<IronOre> {
    fn hint() -> &'static str {
        "Good job on figuring out how to mine iron ore! You can smelt the iron ore into iron ingots using a Furnace, but you won't need to for this tutorial, instead try mining some copper ore using `mine_copper`."
    }
}

impl GuideTopic for Resource<Copper> {
    fn hint() -> &'static str {
        "Awesome! You've made some copper ingots. To win the tutorial, you need to make 1 copper ingot. If you don't have one yet, try mining some copper ore using `mine_copper`, then smelt it into copper ingots using a Furnace."
    }
}

impl<T> GuideTopic for &T
where
    T: GuideTopic,
{
    fn hint() -> &'static str {
        T::hint()
    }
}

impl<Content: ResourceType, const AMOUNT: u32> GuideTopic for Bundle<Content, AMOUNT>
where
    Resource<Content>: GuideTopic,
{
    fn hint() -> &'static str {
        <Resource<Content> as GuideTopic>::hint()
    }
}

/// A guide that provides hints to the player during the tutorial.
#[non_exhaustive]
pub struct Guide;

impl Guide {
    /// Provides a hint about the specified topic and exits the program.
    #[allow(unused_variables)]
    #[allow(private_bounds)]
    pub fn hint<T: GuideTopic>(&self, topic: T) -> ! {
        let message = T::hint();
        println!("{message}");
        process::exit(0);
    }
}

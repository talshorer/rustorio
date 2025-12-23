#![deny(unsafe_code)]

use rustorio::{
    self, Bundle, Recipe, Resource, Tick,
    buildings::{Assembler, Furnace},
    gamemodes::Standard,
    mine_copper, mine_iron,
    recipes::{
        AssemblerRecipe, CopperSmelting, FurnaceRecipe, IronSmelting, PointRecipe, RedScienceRecipe,
    },
    resources::{Copper, CopperOre, Iron, IronOre, Point},
};
use rustorio_engine::research::{RedScience, Technology as _};

// XXX for now those aren't exported...
type FurnaceIronInput = Bundle<Iron, 10>;
type AssemblerIronInput = Bundle<Iron, 15>;
type AssemblerCopperInput = Bundle<Copper, 10>;

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;
type VictoryResources = <GameMode as rustorio::GameMode>::VictoryResources;

#[test]
fn standard_palimpsest() {
    rustorio::play::<GameMode>(user_main);
}

macro_rules! ticklog {
    ($tick: expr, $($tts:tt)*) => {
        println!("[{:05}][:{:04}] {}", $tick.cur(), line!(), format_args!($($tts)*));
    }
}

// REVISIT I don't like that these need to be macros.
// Could we make the API better and write functions?

macro_rules! fill {
    ($tick: expr, $vec: expr, $idx: tt, $resource: expr) => {
        for m in $vec.0.iter_mut() {
            let input = &mut m.inputs($tick).$idx;
            let needed = input.needed_amount();
            if input.amount() <= needed {
                match $resource.split_off(needed) {
                    Ok(res) => input.add(res),
                    Err(_) => break,
                }
            }
        }
    };
}

macro_rules! collect {
    ($tick: expr, $vec: expr, $idx: tt, $resource: expr) => {
        for m in $vec.0.iter_mut() {
            $resource += m.outputs($tick).$idx.empty()
        }
    };
}

macro_rules! empty {
    ($tick: expr, $vec: expr, $idx: tt, $resource: expr) => {
        for m in $vec.0.iter_mut() {
            $resource += m.inputs($tick).$idx.empty()
        }
    };
}

macro_rules! get_idles {
    ($tick: expr, $vec: expr, ($($idx: tt),*)) => {
        $vec.0
            .iter_mut()
            .enumerate()
            .filter_map(|(idx, m)| {
                let inputs = m.inputs($tick);
                (
                    [$(inputs.$idx.amount() > 0),*]
                        .iter()
                        .filter(|b| **b)
                        .count()
                    == 0
                ).then_some(idx)
            })
            .collect::<Vec<_>>()
    };
}

macro_rules! count_working {
    ($tick: expr, $vec: expr, ($($idx: tt),*)) => {
        $vec.0
            .iter_mut()
            .map(|m| {
                let inputs = m.inputs($tick);
                [$(inputs.$idx.amount() < inputs.$idx.needed_amount()),*]
                    .iter()
                    .filter(|b| **b)
                    .count()
                == 0
            })
            .filter(|b| *b)
            .count()
    };
}

// compile-time verification of input type. allows me to use `$recipe::INPUT_AMOUNTS` in
// calculation and be sure the numbers refer to the types I think they do.
macro_rules! input_n_is {
    ($recipe: ty, $idx: tt, $resource_type: ty) => {
        let opt: Option<<$recipe as Recipe>::Inputs> = None;
        if let Some(mut input) = opt {
            input.$idx.add(Resource::<$resource_type>::new_empty());
        }
    };
}

struct FactoryFurnacesInner<R: FurnaceRecipe>(Vec<Furnace<R>>);

impl<R: FurnaceRecipe> Default for FactoryFurnacesInner<R> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<R: FurnaceRecipe> FactoryFurnacesInner<R> {
    fn push<R2: FurnaceRecipe>(&mut self, furnace: Furnace<R2>, recipe: R) {
        self.0.push(furnace.change_recipe(recipe).unwrap());
    }
}

#[derive(Default)]
struct FactoryFurnaces {
    iron: FactoryFurnacesInner<IronSmelting>,
    copper: FactoryFurnacesInner<CopperSmelting>,
}

impl FactoryFurnaces {
    fn len(&self) -> usize {
        self.iron.0.len() + self.copper.0.len()
    }

    // REVISIT allowed to shuffle away the only furnace in a group?
    fn swap_recipe<R1: FurnaceRecipe, R2: FurnaceRecipe + Copy>(
        from: &mut FactoryFurnacesInner<R1>,
        from_indices: Vec<usize>,
        to: &mut FactoryFurnacesInner<R2>,
        recipe: R2,
    ) {
        for idx in from_indices.into_iter().rev() {
            let furnace = from.0.remove(idx);
            to.push(furnace, recipe);
        }
    }

    fn reshuffle(&mut self, tick: &Tick) {
        let idle_iron = get_idles!(tick, self.iron, (0));
        let has_idle_iron = idle_iron.is_empty();

        let idle_copper = get_idles!(tick, self.copper, (0));
        let has_idle_copper = idle_copper.is_empty();

        if has_idle_iron == has_idle_copper {
            return;
        }

        // one of these will be empty and do nothing
        Self::swap_recipe(&mut self.iron, idle_iron, &mut self.copper, CopperSmelting);
        Self::swap_recipe(&mut self.copper, idle_copper, &mut self.iron, IronSmelting);
    }
}

struct FactoryResources {
    iron_ore: Resource<IronOre>,
    iron_ingot: Resource<Iron>,
    copper_ore: Resource<CopperOre>,
    copper_ingot: Resource<Copper>,
    red: Resource<RedScience>,
    point: Resource<Point>,
}

impl FactoryResources {
    fn iron_score(&self) -> u32 {
        self.iron_ore.amount() * IronSmelting::OUTPUT_AMOUNTS.0 / IronSmelting::INPUT_AMOUNTS.0
            + self.iron_ingot.amount()
    }

    fn copper_score(&self) -> u32 {
        self.copper_ore.amount() * CopperSmelting::OUTPUT_AMOUNTS.0
            / CopperSmelting::INPUT_AMOUNTS.0
            + self.copper_ingot.amount()
    }
}

impl Default for FactoryResources {
    fn default() -> Self {
        Self {
            iron_ore: Resource::new_empty(),
            iron_ingot: Resource::new_empty(),
            copper_ore: Resource::new_empty(),
            copper_ingot: Resource::new_empty(),
            red: Resource::new_empty(),
            point: Resource::new_empty(),
        }
    }
}

struct FactoryAssemblersInner<R: AssemblerRecipe>(Vec<Assembler<R>>);

impl<R: AssemblerRecipe> Default for FactoryAssemblersInner<R> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Default)]
struct FactoryAssemblers {
    red: FactoryAssemblersInner<RedScienceRecipe>,
    point: FactoryAssemblersInner<PointRecipe>,
}

struct MineIronSelectorInput {
    iron_score: u32,
    copper_score: u32,
}

#[derive(Default)]
struct Factory {
    furnaces: FactoryFurnaces,
    resources: FactoryResources,
    assmeblers: FactoryAssemblers,
}

// REVISIT how will this look with more than two ore types? and with drills?
impl Factory {
    fn mining(
        &mut self,
        tick: &mut Tick,
        mine_iron_selector: impl FnOnce(MineIronSelectorInput) -> bool,
    ) {
        if mine_iron_selector(MineIronSelectorInput {
            iron_score: self.resources.iron_score(),
            copper_score: self.resources.copper_score(),
        }) {
            ticklog!(tick, "mining iron");
            self.resources.iron_ore.add_bundle(mine_iron::<1>(tick));
        } else {
            ticklog!(tick, "mining copper");
            self.resources.copper_ore.add_bundle(mine_copper::<1>(tick));
        }
    }

    fn fill(&mut self, tick: &Tick, assemblers: bool) {
        fill!(tick, self.furnaces.iron, 0, self.resources.iron_ore);
        fill!(tick, self.furnaces.copper, 0, self.resources.copper_ore);
        if assemblers {
            fill!(tick, self.assmeblers.red, 0, self.resources.iron_ingot);
            fill!(tick, self.assmeblers.red, 1, self.resources.copper_ingot);
            fill!(tick, self.assmeblers.point, 0, self.resources.iron_ingot);
            fill!(tick, self.assmeblers.point, 1, self.resources.copper_ingot);
        }
    }

    fn collect(&mut self, tick: &Tick) {
        collect!(tick, self.furnaces.iron, 0, self.resources.iron_ingot);
        collect!(tick, self.furnaces.copper, 0, self.resources.copper_ingot);
        collect!(tick, self.assmeblers.red, 0, self.resources.red);
        collect!(tick, self.assmeblers.point, 0, self.resources.point);
    }

    fn step(&mut self, tick: &Tick, assemblers: bool) {
        self.collect(tick);
        self.fill(tick, assemblers);

        self.furnaces.reshuffle(tick);
        self.fill(tick, assemblers);
    }

    fn log(&mut self, tick: &Tick) {
        ticklog!(
            tick,
            "iron: ore: {}, ingot: {}, furnaces: {}/{}",
            self.resources.iron_ore.amount(),
            self.resources.iron_ingot.amount(),
            self.furnaces.iron.0.len() - get_idles!(&tick, self.furnaces.iron, (0)).len(),
            self.furnaces.iron.0.len(),
        );
        ticklog!(
            tick,
            "copper: ore: {}, ingot: {}, furnaces: {}/{}",
            self.resources.copper_ore.amount(),
            self.resources.copper_ingot.amount(),
            self.furnaces.copper.0.len() - get_idles!(&tick, self.furnaces.copper, (0)).len(),
            self.furnaces.copper.0.len(),
        );
        ticklog!(
            tick,
            "assemblers: red: {}/{}, point: {}/{}",
            count_working!(tick, self.assmeblers.red, (0, 1)),
            self.assmeblers.red.0.len(),
            count_working!(tick, self.assmeblers.point, (0, 1)),
            self.assmeblers.point.0.len(),
        );
        ticklog!(
            tick,
            "resources: red: {}, point: {}",
            self.resources.red.amount(),
            self.resources.point.amount(),
        );
    }
}

fn build_furnace(tick: &Tick, iron: FurnaceIronInput) -> Furnace<IronSmelting> {
    Furnace::build(tick, IronSmelting, iron)
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, VictoryResources) {
    let StartingResources {
        iron,
        points_technology,
    } = starting_resources;

    let furnace_count: usize = match std::env::var("FURNACES") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => 3,
    };
    let assembler_count: usize = match std::env::var("ASSEMBLERS") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => 1,
    };

    let mut factory = Factory::default();

    factory.resources.iron_ingot += iron;
    tick.log(false);

    while factory.furnaces.len() < furnace_count {
        // REVISIT should I rush to "enough iron for all furnaces" before considering copper?
        factory.mining(
            &mut tick,
            |MineIronSelectorInput {
                 iron_score,
                 copper_score,
             }| {
                iron_score < FurnaceIronInput::AMOUNT
                    || (iron_score - FurnaceIronInput::AMOUNT) * AssemblerCopperInput::AMOUNT
                        <= copper_score * AssemblerIronInput::AMOUNT
            },
        );
        factory.step(&tick, false);

        if let Ok(bundle) = factory.resources.iron_ingot.bundle() {
            let furnace = build_furnace(&tick, bundle);
            if factory.resources.iron_ore.amount() >= IronSmelting::INPUT_AMOUNTS.0
                || factory.resources.copper_ore.amount() < CopperSmelting::INPUT_AMOUNTS.0
            {
                factory.furnaces.iron.push(furnace, IronSmelting);
            } else {
                factory.furnaces.copper.push(furnace, CopperSmelting);
            }
            factory.fill(&tick, false);
        }

        factory.log(&tick);
    }

    while factory.assmeblers.red.0.len() < assembler_count {
        factory.mining(
            &mut tick,
            |MineIronSelectorInput {
                 iron_score,
                 copper_score,
             }| {
                input_n_is!(RedScienceRecipe, 0, Iron);
                input_n_is!(RedScienceRecipe, 1, Copper);
                iron_score < AssemblerIronInput::AMOUNT
                    || (copper_score >= AssemblerCopperInput::AMOUNT
                        && iron_score * RedScienceRecipe::INPUT_AMOUNTS.1
                            <= copper_score * RedScienceRecipe::INPUT_AMOUNTS.0)
            },
        );
        factory.step(&tick, false);

        if factory.resources.iron_ingot.amount() >= AssemblerIronInput::AMOUNT
            && factory.resources.copper_ingot.amount() >= AssemblerCopperInput::AMOUNT
        {
            let assembler = Assembler::build(
                &tick,
                RedScienceRecipe,
                factory.resources.iron_ingot.bundle().unwrap(),
                factory.resources.copper_ingot.bundle().unwrap(),
            );
            factory.assmeblers.red.0.push(assembler);
            factory.fill(&tick, false);
        }

        factory.log(&tick);
    }

    let mut points_technology = Some(points_technology);
    loop {
        if let Ok(bundle) = factory.resources.point.bundle() {
            return (tick, bundle);
        }

        if points_technology.is_some()
            && let Ok(bundle) = factory.resources.red.bundle()
        {
            let points_recipe = points_technology.take().unwrap().research(bundle);
            empty!(
                &tick,
                factory.assmeblers.red,
                0,
                factory.resources.iron_ingot
            );
            empty!(
                &tick,
                factory.assmeblers.red,
                1,
                factory.resources.copper_ingot
            );
            for assembler in std::mem::take(&mut factory.assmeblers.red.0) {
                factory
                    .assmeblers
                    .point
                    .0
                    .push(assembler.change_recipe(points_recipe).unwrap());
            }
        }

        factory.mining(
            &mut tick,
            |MineIronSelectorInput {
                 iron_score,
                 copper_score,
             }| {
                if points_technology.is_some() {
                    input_n_is!(RedScienceRecipe, 0, Iron);
                    input_n_is!(RedScienceRecipe, 1, Copper);
                    iron_score * RedScienceRecipe::INPUT_AMOUNTS.1
                        <= copper_score * RedScienceRecipe::INPUT_AMOUNTS.0
                } else {
                    input_n_is!(PointRecipe, 0, Iron);
                    input_n_is!(PointRecipe, 1, Copper);
                    iron_score * PointRecipe::INPUT_AMOUNTS.1
                        <= copper_score * PointRecipe::INPUT_AMOUNTS.0
                }
            },
        );
        factory.step(&tick, true);
        factory.log(&tick);
    }
}

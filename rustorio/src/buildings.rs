//! Buildings take inputs to produce outputs over time.
//!
//! To use a building, you must first build it which takes a number of resources.
//! Then you can add inputs to it using `add_input` or similar functions.
//! Once it has sufficient inputs, it will start producing outputs, which can be extracted using `take_output` or similar functions.
//!
//! When created, a building is set to a specific [`Recipe`](crate::recipes), which defines the inputs and outputs.
//! This can be changed using the `change_recipe` method, but only if the building is empty (no inputs or outputs).

use std::marker::PhantomData;

use rustorio_engine::resource;

use crate::{
    Bundle, Resource, Tick,
    recipes::{AssemblerRecipe, FurnaceRecipe},
    resources::{Copper, Iron},
};

/// The assembler is used for recipes that require two different inputs to produce an output.
///
/// To use, first build the assembler using [`Assembler::build`], providing the desired recipe and the required resources.
/// Then, add inputs using [`add_input1`](Assembler::add_input1) and [`add_input2`](Assembler::add_input2).
/// The assembler will automatically process the inputs over time, which can be advanced using the [`Tick`].
/// Outputs can be extracted using [`take_output`](Assembler::take_output) or similar.
/// If you want to change the recipe, use [`change_recipe`](Assembler::change_recipe), but ensure the assembler is empty first.
#[derive(Debug)]
pub struct Assembler<R: AssemblerRecipe> {
    input1: Resource<R::Input1>,
    input2: Resource<R::Input2>,
    output: Resource<R::Output>,
    tick: u64,
    crafting_time: u64,
    recipe: PhantomData<R>,
}

impl<R: AssemblerRecipe> Assembler<R> {
    /// Builds an assembler. Costs 15 iron and 10 copper.
    pub fn build(
        tick: &Tick,
        recipe: R,
        iron: Bundle<Iron, 15>,
        copper: Bundle<Copper, 10>,
    ) -> Self {
        let _ = (recipe, iron, copper);
        Self {
            input1: Resource::new_empty(),
            input2: Resource::new_empty(),
            output: Resource::new_empty(),
            tick: tick.cur(),
            crafting_time: 0,
            recipe: PhantomData,
        }
    }

    /// Changes the [`Recipe`](crate::recipes) of the assembler.
    /// Returns the original assembler if the assembler has no inputs or outputs.
    pub fn change_recipe<R2: AssemblerRecipe>(
        self,
        recipe: R2,
    ) -> Result<Assembler<R2>, Assembler<R>> {
        let _ = recipe;
        if self.input1.amount() > 0 || self.input2.amount() > 0 || self.output.amount() > 0 {
            Err(self)
        } else {
            Ok(Assembler {
                input1: Resource::new_empty(),
                input2: Resource::new_empty(),
                output: Resource::new_empty(),
                tick: self.tick,
                crafting_time: 0,
                recipe: PhantomData::<R2>,
            })
        }
    }

    fn tick(&mut self, tick: &Tick) {
        assert!(tick.cur() >= self.tick, "Tick must be non-decreasing");

        self.crafting_time += tick.cur() - self.tick;
        let &count = [
            (self.crafting_time / R::TIME)
                .try_into()
                .expect("Crafting time overflow"),
            self.input1.amount() / R::INPUT1_AMOUNT,
            self.input2.amount() / R::INPUT2_AMOUNT,
        ]
        .iter()
        .min()
        .unwrap();

        self.input1
            .split_off(count * R::INPUT1_AMOUNT)
            .expect("Sufficient input checked above");
        self.input2
            .split_off(count * R::INPUT2_AMOUNT)
            .expect("Sufficient input checked above");
        self.output.add(resource(count * R::OUTPUT_AMOUNT));
        self.crafting_time -= u64::from(count) * R::TIME;

        if self.input1.amount() < R::INPUT1_AMOUNT || self.input2.amount() < R::INPUT2_AMOUNT {
            self.crafting_time = 0;
        }

        self.tick = tick.cur();
    }

    /// Returns a mutable reference to the first input resource.
    /// This allows you to add or remove resources from the input.
    pub fn input1<'a>(&'a mut self, tick: &'a Tick) -> &'a mut Resource<R::Input1> {
        self.tick(tick);
        &mut self.input1
    }

    /// Returns a mutable reference to the second input resource.
    /// This allows you to add or remove resources from the input.
    pub fn input2<'a>(&'a mut self, tick: &'a Tick) -> &'a mut Resource<R::Input2> {
        self.tick(tick);
        &mut self.input2
    }

    /// Returns a mutable reference to the output resource.
    /// This allows you to add or remove resources from the output.
    pub fn output<'a>(&'a mut self, tick: &'a Tick) -> &'a mut Resource<R::Output> {
        self.tick(tick);
        &mut self.output
    }
}

/// The furnace is used to smelt ores into base resources.
///
/// To use, first build the furnace using [`Furnace::build`], providing the desired recipe and the required resources.
/// Then, add inputs using [`add_input`](Furnace::add_input).
/// The furnace will automatically process the inputs over time, which can be advanced using the [`Tick`].
/// Outputs can be extracted using [`take_output`](Furnace::take_output) or similar.
/// If you want to change the recipe, use [`change_recipe`](Furnace::change_recipe), but ensure the furnace is empty first.
#[derive(Debug)]
pub struct Furnace<R: FurnaceRecipe> {
    input: Resource<R::Input>,
    output: Resource<R::Output>,
    tick: u64,
    crafting_time: u64,
    recipe: PhantomData<R>,
}

impl<R: FurnaceRecipe> Furnace<R> {
    /// Builds a furnace. Costs 10 iron.
    pub fn build(tick: &Tick, recipe: R, iron: Bundle<Iron, 10>) -> Self {
        let _ = (recipe, iron);
        Self {
            input: Resource::new_empty(),
            output: Resource::new_empty(),
            tick: tick.cur(),
            crafting_time: 0,
            recipe: PhantomData,
        }
    }

    /// Changes the [`Recipe`](crate::recipes) of the furnace.
    /// Returns the original furnace if the furnace has no inputs or outputs.
    pub fn change_recipe<R2: FurnaceRecipe>(self, recipe: R2) -> Result<Furnace<R2>, Furnace<R>> {
        let _ = recipe;
        if self.input.amount() > 0 || self.output.amount() > 0 {
            Err(self)
        } else {
            Ok(Furnace {
                input: Resource::new_empty(),
                output: Resource::new_empty(),
                tick: self.tick,
                crafting_time: 0,
                recipe: PhantomData::<R2>,
            })
        }
    }

    fn tick(&mut self, tick: &Tick) {
        assert!(tick.cur() >= self.tick, "Tick must be non-decreasing");

        self.crafting_time += tick.cur() - self.tick;
        let count = u32::min(
            (self.crafting_time / R::TIME)
                .try_into()
                .expect("Crafting time overflow"),
            self.input.amount() / R::INPUT_AMOUNT,
        );
        self.input
            .split_off(count * R::INPUT_AMOUNT)
            .expect("Sufficient input checked above");
        self.output.add(resource(count * R::OUTPUT_AMOUNT));
        self.crafting_time -= u64::from(count) * R::TIME;

        if self.input.amount() < R::INPUT_AMOUNT {
            self.crafting_time = 0;
        }

        self.tick = tick.cur();
    }

    /// Returns a mutable reference to the input resource.
    /// This allows you to add or remove resources from the input.
    pub fn input<'a>(&'a mut self, tick: &'a Tick) -> &'a mut Resource<R::Input> {
        self.tick(tick);
        &mut self.input
    }

    /// Returns a mutable reference to the output resource.
    /// This allows you to add or remove resources from the output.
    pub fn output<'a>(&'a mut self, tick: &'a Tick) -> &'a mut Resource<R::Output> {
        self.tick(tick);
        &mut self.output
    }
}

//! Buildings take inputs to produce outputs over time.
//!
//! To use a building, you must first build it which takes a number of resources.
//! Then you can add inputs to it using `add_input` or similar functions.
//! Once it has sufficient inputs, it will start producing outputs, which can be extracted using `take_output` or similar functions.
//!
//! When created, a building is set to a specific [`Recipe`](crate::recipes), which defines the inputs and outputs.
//! This can be changed using the `change_recipe` method, but only if the building is empty (no inputs or outputs).

use std::marker::PhantomData;

use crate::{
    Bundle, Resource, ResourceType,
    recipes::{AssemblerRecipe, FurnaceRecipe},
    tick::Tick,
};

/// The assembler is used for recipes that require two different inputs to produce an output.
#[derive(Debug)]
pub struct Assembler<R: AssemblerRecipe> {
    input1_amount: u32,
    input2_amount: u32,
    output_amount: u32,
    tick: u64,
    start_time: Option<u64>,
    recipe: PhantomData<R>,
}

type AssemblerIronInput = Bundle<{ ResourceType::Iron }, 15>;
type AssemblerCopperInput = Bundle<{ ResourceType::Copper }, 10>;

impl<R: AssemblerRecipe> Assembler<R> {
    /// Builds an assembler. Costs 15 iron and 10 copper.
    pub fn build(tick: &Tick, _iron: AssemblerIronInput, _copper: AssemblerCopperInput) -> Self {
        Self {
            input1_amount: 0,
            input2_amount: 0,
            output_amount: 0,
            tick: tick.cur(),
            start_time: None,
            recipe: PhantomData,
        }
    }

    /// Changes the [`Recipe`](crate::recipes) of the assembler.
    /// Returns the original assembler if the assembler has no inputs or outputs.
    pub fn change_recipe<R2: AssemblerRecipe>(self) -> Result<Assembler<R2>, Assembler<R>> {
        if self.input1_amount > 0 || self.input2_amount > 0 || self.output_amount > 0 {
            Err(self)
        } else {
            Ok(Assembler {
                input1_amount: 0,
                input2_amount: 0,
                output_amount: 0,
                tick: self.tick,
                start_time: None,
                recipe: PhantomData::<R2>,
            })
        }
    }

    fn tick(&mut self, tick: &Tick) {
        assert!(tick.cur() >= self.tick, "Tick must be non-decreasing");
        while self.tick < tick.cur() {
            self.tick += 1;
            if let Some(start_time) = self.start_time
                && self.tick >= start_time + R::TIME
                && self.input1_amount >= R::INPUT1_AMOUNT
                && self.input2_amount >= R::INPUT2_AMOUNT
            {
                self.start_time = None;
                self.input1_amount -= R::INPUT1_AMOUNT;
                self.input2_amount -= R::INPUT2_AMOUNT;
                self.output_amount += R::OUTPUT_AMOUNT;
            }
            if self.start_time.is_none()
                && self.input1_amount >= R::INPUT1_AMOUNT
                && self.input2_amount >= R::INPUT2_AMOUNT
            {
                self.start_time = Some(self.tick);
            }
        }
    }

    /// How much of input resource 1 is currently in the assembler.
    pub fn cur_input1(&mut self, tick: &Tick) -> u32 {
        self.tick(tick);
        self.input1_amount
    }

    /// How much of input resource 2 is currently in the assembler.
    pub fn cur_input2(&mut self, tick: &Tick) -> u32 {
        self.tick(tick);
        self.input2_amount
    }

    /// How much of the output resource is currently in the assembler.
    pub fn cur_output(&mut self, tick: &Tick) -> u32 {
        self.tick(tick);
        self.output_amount
    }

    /// Add some of input resource 1.
    pub fn add_input1<const AMOUNT: u32>(&mut self, tick: &Tick, _ore: Bundle<{ R::INPUT1 }, AMOUNT>) {
        self.tick(tick);
        self.input1_amount += AMOUNT;
    }

    /// Add some of input resource 2.
    pub fn add_input2<const AMOUNT: u32>(&mut self, tick: &Tick, _ore: Bundle<{ R::INPUT2 }, AMOUNT>) {
        self.tick(tick);
        self.input2_amount += AMOUNT;
    }

    /// Take some of input resource 1.
    pub fn take_input1<const AMOUNT: u32>(&mut self, tick: &Tick) -> Option<Bundle<{ R::INPUT1 }, AMOUNT>> {
        self.tick(tick);
        if self.input1_amount >= AMOUNT {
            self.input1_amount -= AMOUNT;
            Some(Bundle::new())
        } else {
            None
        }
    }

    /// Take all of input resource 1 currently in the assembler.
    pub fn empty_input1(&mut self, tick: &Tick) -> Resource<{ R::INPUT1 }> {
        self.tick(tick);
        let amount = self.input1_amount;
        self.input1_amount = 0;
        Resource { amount }
    }

    /// Take some of input resource 2.
    pub fn take_input2<const AMOUNT: u32>(&mut self, tick: &Tick) -> Option<Bundle<{ R::INPUT2 }, AMOUNT>> {
        self.tick(tick);
        if self.input2_amount >= AMOUNT {
            self.input2_amount -= AMOUNT;
            Some(Bundle::new())
        } else {
            None
        }
    }

    /// Take all of input resource 2 currently in the assembler.
    pub fn empty_input2(&mut self, tick: &Tick) -> Resource<{ R::INPUT2 }> {
        self.tick(tick);
        let amount = self.input2_amount;
        self.input2_amount = 0;
        Resource { amount }
    }

    /// Take some of the output resource.
    pub fn take_output<const AMOUNT: u32>(&mut self, tick: &Tick) -> Option<Bundle<{ R::OUTPUT }, AMOUNT>> {
        self.tick(tick);
        if self.output_amount >= AMOUNT {
            self.output_amount -= AMOUNT;
            Some(Bundle::new())
        } else {
            None
        }
    }

    /// Take all of the output resource currently in the assembler.
    pub fn empty_output(&mut self, tick: &Tick) -> Resource<{ R::OUTPUT }> {
        self.tick(tick);
        let amount = self.output_amount;
        self.output_amount = 0;
        Resource { amount }
    }
}

/// The furnace is used to smelt ores into base resources.
#[derive(Debug)]
pub struct Furnace<R: FurnaceRecipe> {
    input_amount: u32,
    output_amount: u32,
    tick: u64,
    start_time: Option<u64>,
    recipe: PhantomData<R>,
}

type FurnaceIronInput = Bundle<{ ResourceType::Iron }, 10>;

impl<R: FurnaceRecipe> Furnace<R> {
    /// Builds a furnace. Costs 10 iron.
    pub fn build(tick: &Tick, _iron: FurnaceIronInput) -> Self {
        Self {
            input_amount: 0,
            output_amount: 0,
            tick: tick.cur(),
            start_time: None,
            recipe: PhantomData,
        }
    }

    /// Changes the [`Recipe`](crate::recipes) of the furnace.
    /// Returns the original furnace if the furnace has no inputs or outputs.
    pub fn change_recipe<R2: FurnaceRecipe>(self) -> Result<Furnace<R2>, Furnace<R>> {
        if self.input_amount > 0 || self.output_amount > 0 {
            Err(self)
        } else {
            Ok(Furnace {
                input_amount: 0,
                output_amount: 0,
                tick: self.tick,
                start_time: None,
                recipe: PhantomData::<R2>,
            })
        }
    }

    fn tick(&mut self, tick: &Tick) {
        assert!(tick.cur() >= self.tick, "Tick must be non-decreasing");
        while self.tick < tick.cur() {
            self.tick += 1;
            if let Some(start_time) = self.start_time
                && self.tick >= start_time + R::TIME
                && self.input_amount >= R::INPUT_AMOUNT
            {
                self.start_time = None;
                self.input_amount -= R::INPUT_AMOUNT;
                self.output_amount += R::OUTPUT_AMOUNT;
            }
            if self.start_time.is_none() && self.input_amount >= R::INPUT_AMOUNT {
                self.start_time = Some(self.tick);
            }
        }
    }

    pub fn cur_input(&mut self, tick: &Tick) -> u32 {
        self.tick(tick);
        self.input_amount
    }

    pub fn cur_output(&mut self, tick: &Tick) -> u32 {
        self.tick(tick);
        self.output_amount
    }

    pub fn add_input<const AMOUNT: u32>(&mut self, tick: &Tick, _ore: Bundle<{ R::INPUT }, AMOUNT>) {
        self.tick(tick);
        self.input_amount += AMOUNT;
    }

    pub fn take_input<const AMOUNT: u32>(&mut self, tick: &Tick) -> Option<Bundle<{ R::INPUT }, AMOUNT>> {
        self.tick(tick);
        if self.input_amount >= AMOUNT {
            self.input_amount -= AMOUNT;
            Some(Bundle::new())
        } else {
            None
        }
    }

    pub fn empty_input(&mut self, tick: &Tick) -> Resource<{ R::INPUT }> {
        self.tick(tick);
        let amount = self.input_amount;
        self.input_amount = 0;
        Resource { amount }
    }

    pub fn take_output<const AMOUNT: u32>(&mut self, tick: &Tick) -> Option<Bundle<{ R::OUTPUT }, AMOUNT>> {
        self.tick(tick);
        if self.output_amount >= AMOUNT {
            self.output_amount -= AMOUNT;
            Some(Bundle::new())
        } else {
            None
        }
    }

    pub fn empty_output(&mut self, tick: &Tick) -> Resource<{ R::OUTPUT }> {
        self.tick(tick);
        let amount = self.output_amount;
        self.output_amount = 0;
        Resource { amount }
    }
}

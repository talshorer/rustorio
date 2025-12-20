//! Recipes define all item transformations in the game via input items, output items, and time.

use std::marker::PhantomData;

pub use rustorio_derive::Recipe;

use crate::{
    ResourceType,
    resources::{Bundle, InsufficientResourceError, Resource},
};

/// One recipe item and its current amount inside a machine's input/output buffer.
#[derive(Debug)]
pub struct RecipeItem<const AMOUNT: usize, R: ResourceType> {
    _phantom: PhantomData<[R; AMOUNT]>,
    /// Current amount of this item in a machine's buffer.
    amount: u32,
}

impl<const AMOUNT: usize, R: ResourceType> Default for RecipeItem<AMOUNT, R> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
            amount: Default::default(),
        }
    }
}

impl<const NEEDED_AMOUNT: usize, R: ResourceType> RecipeItem<NEEDED_AMOUNT, R> {
    /// Needed amount of the resource for one cycle of its recipe.
    pub const fn needed_amount(&self) -> u32 {
        NEEDED_AMOUNT as u32
    }

    /// How much of the resource is currently in the buffer.
    pub fn cur(&self) -> u32 {
        self.amount
    }

    /// Consumes a [`Resource`] and puts the contained resources into the buffer.
    pub fn add(&mut self, item: impl Into<Resource<R>>) {
        self.amount += item.into().amount()
    }

    /// Takes a specified amount of resource from this buffer and puts it into a [`Resource`].
    pub fn take(&mut self, amount: u32) -> Result<Resource<R>, InsufficientResourceError<R>> {
        if self.amount >= amount {
            self.amount -= amount;
            Ok(Resource::new(amount))
        } else {
            Err(InsufficientResourceError::new(amount, self.amount))
        }
    }

    /// Takes a specified amount of resource from this buffer and puts it into a [`Bundle`].
    pub fn take_bundle<const BUNDLE_AMOUNT: u32>(
        &mut self,
    ) -> Result<Bundle<R, BUNDLE_AMOUNT>, InsufficientResourceError<R>> {
        if self.amount >= BUNDLE_AMOUNT {
            self.amount -= BUNDLE_AMOUNT;
            Ok(Bundle::new())
        } else {
            Err(InsufficientResourceError::new(BUNDLE_AMOUNT, self.amount))
        }
    }

    /// Takes all resource currently in the buffer and puts it into a [`Resource`].
    pub fn empty(&mut self) -> Resource<R> {
        Resource::new(std::mem::take(&mut self.amount))
    }
}

/// Get a mutable reference to the inner amount.
/// This is not a function of `RecipeItem` to allow mods to choose to not export it to user code.
pub fn recipe_item_amount<const AMOUNT: usize, Resource: ResourceType>(
    item: &mut RecipeItem<AMOUNT, Resource>,
) -> &mut u32 {
    &mut item.amount
}

/// Basic recipe trait. A building's specific recipe trait can then be defined like
/// ```rust
/// trait AssemblerRecipe: rustorio_engine::recipe::Recipe + rustorio_engine::Sealed {}
/// ```
/// For example, one could define a recipe that takes three inputs and gives two outputs like:
/// ```rust
/// use rustorio_engine::{recipe::Recipe, resource_type};
///
/// resource_type!(Resource1);
/// resource_type!(Resource2);
/// resource_type!(Resource3);
/// resource_type!(Resource4);
/// resource_type!(Resource5);
///
/// #[derive(Recipe)]
/// #[recipe_inputs(
///     (10, Resource1),
///     (5, Resource2),
///     (1, Resource3),
/// )]
/// #[recipe_outputs(
///     (1, Resource4),
///     (100, Resource5),
/// )]
/// #[recipe_ticks(10)]
/// pub struct ThreeToTwoRecipe;
/// ```
/// The recipe will then take 10 ticks per cycle, consuming 10 `Resource1`, 5 `Resource2`,
/// and 1 `Resource3`, and produce 1 `Resource4` and 100 `Resource5`.
pub trait Recipe {
    /// Amount of ticks one cycle of the recipe takes to complete.
    const TIME: u64;

    /// Typically a tuple of multiple `RecipeTypes`, to define the inputs
    /// for one cycle of the recipe.
    type Inputs: std::fmt::Debug;

    /// Typically a tuple of multiple `RecipeTypes`, to define the outputs
    /// for one cycle of the recipe.
    type Outputs: std::fmt::Debug;

    /// The type for `Self::InputAmountsType`, which is used to allow users to
    /// access the input amount for each of the input resource types, per recipe cycle.
    type InputAmountsType: std::fmt::Debug;

    /// Amount for each of the input resource types, per recipe cycle.
    const INPUT_AMOUNTS: Self::InputAmountsType;

    /// The type for `Self::OuptutAmountsType`, which is used to allow users to
    /// access the output amount for each of the output resource types, per recipe cycle.
    type OutputAmountsType: std::fmt::Debug;

    /// Amount for each of the output resource types, per recipe cycle.
    const OUTPUT_AMOUNTS: Self::OutputAmountsType;

    /// Factory function to create a new `Self::Inputs` with zero resources.
    fn new_inputs() -> Self::Inputs;

    /// Factory function to create a new `Self::Outputs` with zero resources.
    fn new_outputs() -> Self::Outputs;

    /// Iterator helper over `Self::Inputs`.
    fn iter_inputs(items: &mut Self::Inputs) -> impl Iterator<Item = (u32, &mut u32)>;

    /// Iterator helper over `Self::Outputs`.
    fn iter_outputs(items: &mut Self::Outputs) -> impl Iterator<Item = (u32, &mut u32)>;
}

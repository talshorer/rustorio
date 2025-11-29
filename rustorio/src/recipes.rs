//! A recipe is a way of turning resources into other resources.
//! A specific recipe specifies the input and output resources, as well as the time it takes to complete the recipe.

use std::fmt::Debug;

use crate::{ResourceType, sealed::Sealed};

/// Any recipe that implements this trait can be used in an [`Assembler`](crate::buildings::Assembler).
pub trait AssemblerRecipe: Debug + Sealed {
    /// The first of the two types of resources consumed by this recipe.
    const INPUT1: ResourceType;
    /// The amount of the first input resource consumed by this recipe.
    const INPUT1_AMOUNT: u32;
    /// The second of the two types of resources consumed by this recipe.
    const INPUT2: ResourceType;
    /// The amount of the second input resource consumed by this recipe.
    const INPUT2_AMOUNT: u32;
    /// The type of resource produced by this recipe.
    const OUTPUT: ResourceType;
    /// The amount of the output resource produced by this recipe.
    const OUTPUT_AMOUNT: u32;
    /// The time (in ticks) it takes to complete this recipe.
    const TIME: u64;
}

/// A recipe for crafting red science packs.
#[derive(Debug, Clone, Copy)]
pub struct RedScienceRecipe;
impl Sealed for RedScienceRecipe {}
impl AssemblerRecipe for RedScienceRecipe {
    const INPUT1: ResourceType = ResourceType::Iron;
    const INPUT1_AMOUNT: u32 = 1;
    const INPUT2: ResourceType = ResourceType::Copper;
    const INPUT2_AMOUNT: u32 = 1;
    const OUTPUT: ResourceType = ResourceType::RedScience;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 5;
}

/// The recipe you need to win! An [`Assembler`](crate::buildings::Assembler) recipe that creates points. Converts 4 iron and 4 copper into 1 point resource. Takes 20 ticks.
///
/// You can unlock this recipe by researching [`PointsResearch`](crate::research::PointsResearch).
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PointRecipe;

impl Sealed for PointRecipe {}

impl AssemblerRecipe for PointRecipe {
    const INPUT1: ResourceType = ResourceType::Iron;
    const INPUT1_AMOUNT: u32 = 4;
    const INPUT2: ResourceType = ResourceType::Copper;
    const INPUT2_AMOUNT: u32 = 4;
    const OUTPUT: ResourceType = ResourceType::Point;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 20;
}

/// Any recipe that implements this trait can be used in a [`Furnace`](crate::buildings::Furnace).
pub trait FurnaceRecipe: Debug + Sealed {
    const INPUT: ResourceType;
    const INPUT_AMOUNT: u32;
    const OUTPUT: ResourceType;
    const OUTPUT_AMOUNT: u32;
    const TIME: u64;
}

/// A [`Furnace`](crate::buildings::Furnace) recipe that smelts iron ore into iron. Converts 2 iron ore into 1 iron. Takes 10 ticks.
#[derive(Debug, Clone, Copy)]
pub struct IronSmelting;

impl Sealed for IronSmelting {}

impl FurnaceRecipe for IronSmelting {
    const INPUT: ResourceType = ResourceType::IronOre;
    const INPUT_AMOUNT: u32 = 2;
    const OUTPUT: ResourceType = ResourceType::Iron;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 10;
}

/// A [`Furnace`](crate::buildings::Furnace) recipe that smelts copper ore into copper. Converts 2 copper ore into 1 copper. Takes 10 ticks.
#[derive(Debug, Clone, Copy)]
pub struct CopperSmelting;

impl Sealed for CopperSmelting {}

impl FurnaceRecipe for CopperSmelting {
    const INPUT: ResourceType = ResourceType::CopperOre;
    const INPUT_AMOUNT: u32 = 2;
    const OUTPUT: ResourceType = ResourceType::Copper;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 10;
}

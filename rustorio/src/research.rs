use crate::{Bundle, ResourceType, recipes::PointRecipe, sealed::Sealed};

type RedScienceBundle<const AMOUNT: u32> = Bundle<{ ResourceType::RedScience }, AMOUNT>;

/// A research can be carried out by calling the `research` method with the required science packs.
/// This will consume the science packs and the research itself, and return whatever the research unlocks, mostly recipes and other researches.
pub trait Research: Sealed {
    const RED_SCIENCE_COST: u32;
    type Unlocks;

    fn research(self, _red_science: RedScienceBundle<{ Self::RED_SCIENCE_COST }>) -> Self::Unlocks;
}

/// Research that unlocks the ability to produce points.
#[non_exhaustive]
pub struct PointsResearch;
impl Sealed for PointsResearch {}
impl Research for PointsResearch {
    const RED_SCIENCE_COST: u32 = 5;
    type Unlocks = PointRecipe;

    fn research(self, _red_science: RedScienceBundle<{ Self::RED_SCIENCE_COST }>) -> Self::Unlocks {
        PointRecipe {}
    }
}

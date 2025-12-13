//! Researches are technologies that can be unlocked by consuming science packs.
//! They usually unlock new recipes or further researches.
//!
//! This module defines the researches available in Rustorio.

use rustorio_engine::research::{RedScience, Research};

use crate::{Bundle, recipes::PointRecipe};

/// Research that unlocks the ability to produce points.
#[non_exhaustive]
pub struct PointsResearch;
impl rustorio_engine::Sealed for PointsResearch {}

impl Research for PointsResearch {
    const RED_SCIENCE_COST: u32 = 5;
    type Unlocks = PointRecipe;

    fn research(self, red_science: Bundle<RedScience, { Self::RED_SCIENCE_COST }>) -> Self::Unlocks {
        let _ = red_science;
        PointRecipe {}
    }
}

//! Researches are technologies that can be unlocked by consuming science packs.
//! They usually unlock new recipes or further researches.
//!
//! This module defines the the science pack resources and the `Research` trait.

use crate::{Sealed, resource_type, resources::Bundle};

resource_type!(
    /// Red science packs used for research.
    RedScience);

/// A research can be carried out by calling the `research` method with the required science packs.
/// This will consume the science packs and the research itself, and return whatever the research unlocks, mostly recipes and other researches.
pub trait Research: Sealed {
    /// The amount of red science required to carry out this research.
    const RED_SCIENCE_COST: u32;
    /// The reward for completing this research.
    type Unlocks;

    /// Carries out the research by consuming the required science packs and the research itself, returning whatever this research unlocks.
    fn research(self, red_science: Bundle<RedScience, { Self::RED_SCIENCE_COST }>) -> Self::Unlocks;
}

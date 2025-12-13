//! Technologies can be unlocked by consuming science packs.
//! They usually unlock new recipes or further technologies.
//!
//! This module defines the the science pack resources and the `Technology` trait.

use crate::{Sealed, resource_type, resources::Bundle};

resource_type!(
    /// Red science packs used for research.
    RedScience);

/// A technology can be unlocked out by calling the `research` method with the required science packs.
/// This will consume the science packs and the technology itself, and return whatever the technology unlocks, mostly recipes and other technologies.
pub trait Technology: Sealed {
    /// The amount of red science required to carry out this technology.
    const RED_SCIENCE_COST: u32;
    /// The reward for completing this technology.
    type Unlocks;

    /// Carries out the research by consuming the required science packs and the research itself, returning whatever this research unlocks.
    fn research(self, red_science: Bundle<RedScience, { Self::RED_SCIENCE_COST }>) -> Self::Unlocks;
}

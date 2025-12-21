//! Technologies can be unlocked by consuming science packs.
//! They usually unlock new recipes or further technologies.
//!
//! For example, if you want to produce points using the `PointRecipe` recipe,
//! you must first unlock it by researching the `PointsTechnology` technology.
//!
//! This module defines the technologies available in Rustorio.

use rustorio_engine::research::{RedScience, Technology};

use crate::{Bundle, recipes::PointRecipe};

/// Technology that unlocks the ability to produce points.
#[non_exhaustive]
pub struct PointsTechnology;
impl rustorio_engine::Sealed for PointsTechnology {}

impl Technology for PointsTechnology {
    const RED_SCIENCE_COST: u32 = 5;
    type Unlocks = PointRecipe;

    fn research(
        self,
        red_science: Bundle<RedScience, { Self::RED_SCIENCE_COST }>,
    ) -> Self::Unlocks {
        let _ = red_science;
        PointRecipe {}
    }
}

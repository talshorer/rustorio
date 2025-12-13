//! A game mode defines the starting resources and victory conditions for a game.
//!
//! This module defines the available game modes.

use rustorio_engine::{
    bundle,
    gamemodes::{GameMode, StartingResources},
};

use crate::{
    Bundle,
    guide::Guide,
    research::PointsTechnology,
    resources::{Copper, Iron, Point},
};

/// Starting resources for the tutorial game mode.
pub struct TutorialStartingResources {
    /// Initial iron bundle.
    pub iron: Bundle<Iron, 10>,
    /// The in-game guide that provides hints to the player.
    pub guide: Guide,
}

impl StartingResources for TutorialStartingResources {
    fn init() -> Self {
        Self {
            iron: bundle(),
            guide: Guide,
        }
    }
}

/// The tutorial game mode. Very short distance from start to victory, meant to teach the very most basic elements of play.
pub struct Tutorial;

impl GameMode for Tutorial {
    type StartingResources = TutorialStartingResources;
    type VictoryResources = Bundle<Copper, 4>;
}

/// Starting resources for the standard game mode. Includes 10 iron and the ability to research points.
pub struct StandardStartingResources {
    /// Initial iron bundle.
    pub iron: Bundle<Iron, 10>,
    /// The tech you must research to start creating the game winning points.
    pub points_technology: PointsTechnology,
}
impl StartingResources for StandardStartingResources {
    fn init() -> Self {
        Self {
            iron: bundle(),
            points_technology: PointsTechnology,
        }
    }
}

/// The standard game mode. Forces the player through the main gameplay mechanics.
pub struct Standard;

impl GameMode for Standard {
    type StartingResources = StandardStartingResources;
    type VictoryResources = Bundle<Point, 10>;
}

use crate::{
    Bundle,
    guide::Guide,
    research::PointsResearch,
    resources::{Copper, Iron, Point},
};

pub(crate) trait StartingResources {
    fn init() -> Self;
}

/// A game mode defines the starting resources and victory conditions for a game.
pub trait GameMode {
    /// Starting resources provided to the player at the beginning of the game.
    #[allow(private_bounds)]
    type StartingResources: StartingResources;
    /// Resources required to achieve victory.
    type VictoryResources;
}

/// Starting resources for the tutorial game mode.
pub struct TutorialStartingResources {
    pub iron: Bundle<Iron, 10>,
    pub guide: Guide,
}

impl StartingResources for TutorialStartingResources {
    fn init() -> Self {
        Self {
            iron: Bundle::new(),
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
    pub iron: Bundle<Iron, 10>,
    pub points_research: PointsResearch,
}
impl StartingResources for StandardStartingResources {
    fn init() -> Self {
        Self {
            iron: Bundle::new(),
            points_research: PointsResearch,
        }
    }
}

/// The standard game mode. Forces the player through the main gameplay mechanics.
pub struct Standard;

impl GameMode for Standard {
    type StartingResources = StandardStartingResources;
    type VictoryResources = Bundle<Point, 10>;
}

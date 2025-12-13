pub trait StartingResources {
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

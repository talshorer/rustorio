use crate::{Bundle, ResourceType};

pub(crate) trait StartingResources {
    fn init() -> Self;
}

pub trait GameMode {
    #[allow(private_bounds)]
    type StartingResources: StartingResources;
    type VictoryResources;
}

pub struct TutorialStartingResources {
    pub iron: Bundle<{ ResourceType::Iron }, 10>,
}

impl StartingResources for TutorialStartingResources {
    fn init() -> Self {
        Self { iron: Bundle::new() }
    }
}

pub struct Tutorial;

impl GameMode for Tutorial {
    type StartingResources = TutorialStartingResources;
    type VictoryResources = Bundle<{ ResourceType::Copper }, 1>;
}

pub struct StandardStartingResources {
    pub iron: Bundle<{ ResourceType::Iron }, 10>,
}
impl StartingResources for StandardStartingResources {
    fn init() -> Self {
        Self { iron: Bundle::new() }
    }
}
pub struct Standard;

impl GameMode for Standard {
    type StartingResources = StandardStartingResources;
    type VictoryResources = Bundle<{ ResourceType::Point }, 10>;
}

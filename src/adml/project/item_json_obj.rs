use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ItemJsonObj {
    pub id: String, // used for internal identification
    pub display_name: String, // used for in-game display
    #[serde(default)]
    pub stack_size: StackSize,
    pub interaction: Interaction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StackSize(u32);
impl Default for StackSize {
    fn default() -> Self {
        StackSize(1)
    }
}

use interactions::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Interaction {
    pub consume: Option<Consume>,
    pub right_click: Option<RightClick>,
}

pub mod interactions {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Consume {
        pub duration: u32,
        pub trigger: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RightClick {
        #[serde(default)]
        pub cooldown: RightClickCooldown,
        pub trigger: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RightClickCooldown {
        pub value: u32,
        pub reset_on_release: bool,
    }

    impl Default for RightClickCooldown {
        fn default() -> Self {
            Self {
                value: 32,
                reset_on_release: true,
            }
        }
    }
}
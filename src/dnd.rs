use std::sync::Arc;

use strum::{Display, EnumIter};

#[derive(Default, Debug, EnumIter, Display, Clone, PartialEq, Eq, Copy)]
pub enum Dice {
    D4,
    D6,
    #[default]
    D8,
    D10,
    D12,
    D20,
    Other(u8),
}

#[derive(EnumIter, Display, Debug, Clone, PartialEq, Eq)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
    Artificer,
    Homebrew {
        name: Option<Arc<str>>,
        hit_dice: Dice,
    },
}

use std::sync::Arc;

use strum::{Display, EnumIter, IntoEnumIterator};

use crate::prelude::*;

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

impl Dice {
    pub fn prompt(prompt: &str) -> anyhow::Result<Dice> {
        let dice = select(prompt, Dice::iter().collect())?;
        let dice = match dice {
            Dice::Other(_) => input("Enter the number of sides on your custom dice")
                .and_then(|s| s.parse::<u8>().anyhow())
                .map(Dice::Other)?,
            dice => dice,
        };
        Ok(dice)
    }
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

impl Class {
    pub fn prompt() -> anyhow::Result<Class> {
        let class = select("Select a class", Class::iter().collect())?;
        let class = match class {
            Class::Homebrew { .. } => {
                let dice = Dice::prompt("Select the hit dice for your homebrew class")?;
                let name = input("Enter the name of your homebrew class")?;
                let name = arc_str(name);
                Class::Homebrew {
                    name: Some(name),
                    hit_dice: dice,
                }
            }
            _ => class,
        };
        Ok(class)
    }
}

use std::{str::FromStr, sync::Arc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::tuple,
};
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
    pub fn prompt(prompt: &str) -> Dice {
        let dice = select(prompt, Dice::iter().collect());
        let dice = match dice {
            Dice::Other(_) => input_map(
                "Enter the number of sides of the hit dice: ",
                Dice::from_str,
            ),
            dice => dice,
        };
        dice
    }
}

impl From<Dice> for f32 {
    fn from(value: Dice) -> Self {
        f32::from(u8::from(value))
    }
}

impl From<Dice> for u8 {
    fn from(value: Dice) -> Self {
        match value {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
            Dice::Other(other) => other,
        }
    }
}

#[derive(Debug)]
pub struct DiceParseError;

impl std::fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid dice")
    }
}

impl std::error::Error for DiceParseError {}

impl FromStr for Dice {
    type Err = DiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_dice(s: &str) -> nom::IResult<&str, u8> {
            map_res(
                alt((
                    map(
                        tuple((tag("d"), digit1)),
                        |(_d_tag, faces): (&str, &str)| faces,
                    ),
                    digit1,
                )),
                str::parse::<u8>,
            )(s)
        }
        let (_, parsed) = parse_dice(s).map_err(|_| DiceParseError)?;
        match parsed {
            4 => Ok(Dice::D4),
            6 => Ok(Dice::D6),
            8 => Ok(Dice::D8),
            10 => Ok(Dice::D10),
            12 => Ok(Dice::D12),
            20 => Ok(Dice::D20),
            other => Ok(Dice::Other(other)),
        }
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
    pub fn prompt() -> Class {
        let class = select("Select a class", Class::iter().collect());

        match class {
            Class::Homebrew { .. } => {
                let dice = Dice::prompt("Select the hit dice for your homebrew class");
                let name = input("Enter the name of your homebrew class");
                let name = arc_str(name);
                Class::Homebrew {
                    name: Some(name),
                    hit_dice: dice,
                }
            }
            _ => class,
        }
    }
    pub fn hit_dice(&self) -> Dice {
        match self {
            Class::Barbarian => Dice::D12,
            Class::Bard => Dice::D8,
            Class::Cleric => Dice::D8,
            Class::Druid => Dice::D8,
            Class::Fighter => Dice::D10,
            Class::Monk => Dice::D8,
            Class::Paladin => Dice::D10,
            Class::Ranger => Dice::D10,
            Class::Rogue => Dice::D8,
            Class::Sorcerer => Dice::D6,
            Class::Warlock => Dice::D8,
            Class::Wizard => Dice::D6,
            Class::Artificer => Dice::D8,
            Class::Homebrew { hit_dice, .. } => *hit_dice,
        }
    }
}

use std::iter::repeat;
use std::ops::Div;

use strum::{Display, EnumIter, IntoEnumIterator};

use crate::dnd::Class;

use crate::prelude::*;

pub fn calculate_hp() -> anyhow::Result<()> {
    let class = Class::prompt();
    let level = input_map("Level: ", str::parse::<u32>);
    let con_mod = input_map("Constiution modifier: ", str::parse::<u8>);
    let has_tough = confirm("Do you have the Tough feat?");
    let is_hill_dwarf = confirm("Are you a Hill Dwarf?");
    let method = select("Choose a method:", Method::iter().collect());

    let hp = Hp {
        class,
        level,
        con_mod,
        has_tough,
        is_hill_dwarf,
        method,
    };

    tracing::debug!(?hp);

    let hp = hp.calculate();
    match hp {
        HpResult::Rolled(hp, rolls) => {
            tracing::info!("Rolls: {:?}", rolls);
            tracing::info!("HP: {}", hp);
        }
        HpResult::Average(hp) => tracing::info!("HP: {}", hp),
    }

    Ok(())
}

#[derive(Debug, Display, EnumIter, Clone)]
enum Method {
    Rolled,
    Average,
}

#[derive(Debug)]
struct Hp {
    class: Class,
    level: u32,
    con_mod: u8,
    has_tough: bool,
    is_hill_dwarf: bool,
    method: Method,
}

#[derive(Debug, PartialEq)]
enum HpResult {
    Rolled(f32, Vec<u8>),
    Average(f32),
}

impl Hp {
    fn calculate(&self) -> HpResult {
        use rand::Rng;

        let tough_value = if self.has_tough { 2 * self.level } else { 0 } as f32;
        let hill_dwarf_value = if self.is_hill_dwarf { self.level } else { 0 } as f32;
        let hit_dice = f32::from(self.class.hit_dice());

        match self.method {
            Method::Rolled => {
                let rolls = repeat(()).take(self.level as usize - 1).map(|_| {
                    let roll = rand::thread_rng().gen_range(1..=hit_dice as u8);
                    tracing::trace!("Rolled a {}", roll);
                    roll as f32 + self.con_mod as f32
                });
                let hp = hit_dice
                    + self.con_mod as f32
                    + tough_value
                    + hill_dwarf_value
                    + rolls.clone().sum::<f32>();
                HpResult::Rolled(hp, rolls.map(|x| x as u8).collect())
            }
            Method::Average => {
                let avg = 1.0 + hit_dice.div(2.0).ceil();

                tracing::trace!(
                    "lvl 1 ({} hit dice + {} CON) = {}",
                    hit_dice,
                    self.con_mod,
                    hit_dice + self.con_mod as f32
                );
                tracing::trace!(
                    "HP at subsequent levels = {}",
                    (avg + self.con_mod as f32)
                        + f32::from(self.has_tough)
                        + f32::from(self.is_hill_dwarf) * 2.0
                );

                let hp = hit_dice
                    + self.con_mod as f32
                    + (avg + self.con_mod as f32) * (self.level - 1) as f32
                    + tough_value
                    + hill_dwarf_value;
                HpResult::Average(hp)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test_case]
    fn test_hp_barbarian() -> TResult {
        test(|| -> anyhow::Result<HpResult> {
            let hp = Hp {
                class: Class::Barbarian,
                level: 1,
                con_mod: 2,
                has_tough: false,
                is_hill_dwarf: false,
                method: Method::Average,
            };

            let hp = hp.calculate();
            assert_eq!(hp, HpResult::Average(14.0));
            Ok(hp)
        })
    }

    #[test_case]
    fn test_calculate_hp_fighter() -> TResult {
        test(|| {
            let hp = Hp {
                class: Class::Fighter,
                level: 5,
                con_mod: 2,
                has_tough: false,
                is_hill_dwarf: false,
                method: Method::Average,
            };
            let hp = hp.calculate();
            assert_eq!(hp, HpResult::Average(44.0));
            hp
        })
    }

    #[test_case]
    fn test_calculate_hp_wizard() -> TResult {
        test(|| {
            let hp = Hp {
                class: Class::Wizard,
                level: 3,
                con_mod: 1,
                has_tough: true,
                is_hill_dwarf: true,
                method: Method::Average,
            };
            let hp = hp.calculate();
            assert_eq!(hp, HpResult::Average(26.0));
            hp
        })
    }
}

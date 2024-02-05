use strum::IntoEnumIterator;

use crate::dnd::{Class, Dice};
use crate::prelude::*;

pub fn calculate_hp() -> anyhow::Result<()> {
    let class = prompt_class()?;
    tracing::info!(?class);
    Ok(())
}

pub fn prompt_class() -> anyhow::Result<Class> {
    let class = select("Select a class", Class::iter().collect())?;
    let class = match class {
        Class::Homebrew { .. } => {
            let dice = prompt_dice("Select the hit dice for your homebrew class")?;
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

pub fn prompt_dice(prompt: &str) -> anyhow::Result<Dice> {
    let dice = select(prompt, Dice::iter().collect())?;
    let dice = match dice {
        Dice::Other(_) => input("Enter the number of sides on your custom dice")
            .and_then(|s| s.parse::<u8>().anyhow())
            .map(Dice::Other)?,
        dice => dice,
    };
    Ok(dice)
}

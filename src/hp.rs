use crate::dnd::Class;

use crate::prelude::*;

pub fn calculate_hp() -> anyhow::Result<()> {
    let class = Class::prompt();
    let level = input_map("Level: ", str::parse::<u32>);
    let con_mod = input_map("Constiution modifier: ", str::parse::<u8>);
    let has_tough = confirm("Do you have the Tough feat?");

    tracing::info!(?class, ?level, ?con_mod, ?has_tough);
    Ok(())
}

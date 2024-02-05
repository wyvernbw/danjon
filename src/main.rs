#![feature(lazy_cell)]
use std::fmt::Display;

use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::select;

mod dnd;
mod hp;
mod prelude;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let tool = select("What would you like to do?", Tool::iter().collect())?;

    tracing::info!(?tool);

    match tool {
        Tool::CalculateHp => hp::calculate_hp()?,
        Tool::CalculateAc => todo!(),
    }

    Ok(())
}

#[derive(Debug, EnumIter)]
enum Tool {
    CalculateHp,
    CalculateAc,
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

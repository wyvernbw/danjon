#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]

use std::fmt::Display;

use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::select;

mod dnd;
mod hp;
mod prelude;
mod testing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let tool = select("What would you like to do?", Tool::iter().collect());

    tracing::info!(?tool);

    match tool {
        Tool::CalculateHp => hp::calculate_hp()?,
        Tool::CalculateAc => todo!(),
    }

    Ok(())
}

#[derive(Debug, EnumIter, Clone)]
enum Tool {
    CalculateHp,
    CalculateAc,
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

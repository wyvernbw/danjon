#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]

use std::fmt::Display;

use crossterm::{execute, terminal::ClearType};
use strum::{EnumIter, IntoEnumIterator};
use tracing::Subscriber;
use tracing_subscriber::{filter::filter_fn, Layer};

use crate::prelude::select;

mod ac;
mod dnd;
mod hp;
mod prelude;
mod testing;

#[cfg(debug_assertions)]
fn subscriber() -> impl tracing::Subscriber {
    tracing_subscriber::FmtSubscriber::default()
}

#[cfg(not(debug_assertions))]
fn subscriber() -> impl tracing::Subscriber {
    use tracing_subscriber::layer::SubscriberExt;

    tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_ansi(false)
            .pretty()
            .with_level(false)
            .with_file(false)
            .with_line_number(false)
            .without_time()
            .with_filter(filter_fn(|metadata| {
                metadata.level() <= &tracing::Level::INFO
            })),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(subscriber())?;

    let tool = select("What would you like to do?", Tool::iter().collect());

    tracing::debug!(?tool);

    loop {
        match tool {
            Tool::CalculateHp => hp::calculate_hp()?,
            Tool::CalculateAc => ac::calculate_ac()?,
        }
        let again = select(
            "What shall be your next destination?",
            vec!["Main menu", "Exit"],
        );
        match again {
            "Main menu" => {
                execute!(
                    std::io::stdout(),
                    crossterm::terminal::Clear(ClearType::All)
                )?;
            }
            "Exit" => std::process::exit(0),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, EnumIter, Clone)]
enum Tool {
    CalculateHp,
    CalculateAc,
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Tool::CalculateHp => "Calculate HP".to_string(),
            Tool::CalculateAc => "Calculate AC".to_string(),
        };
        write!(f, "{}", string)
    }
}

use yansi::Paint;

use crate::dnd::Class;

use crate::prelude::*;

pub fn calculate_hp() -> anyhow::Result<()> {
    let class = Class::prompt();
    fn get_con_mod() -> u32 {
        match input("Constitution modifier: ").and_then(|s| s.parse::<u32>().anyhow()) {
            Ok(con_mod) => con_mod,
            Err(err) => {
                tracing::error!(?err);
                println!("{}", Paint::red(err));
                get_con_mod()
            }
        }
    }
    let con_mod = get_con_mod();
    tracing::info!(?class);
    Ok(())
}

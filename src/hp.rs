use crate::dnd::Class;

pub fn calculate_hp() -> anyhow::Result<()> {
    let class = Class::prompt();
    tracing::info!(?class);
    Ok(())
}

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let options = vec![Tool::CalculateHp, Tool::CalculateAc];
    let answer = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .vim_mode(true)
        .interact()?;
    let answer = &options[answer];

    tracing::info!(?answer);
    Ok(())
}

#[derive(Debug)]
enum Tool {
    CalculateHp,
    CalculateAc,
}

impl ToString for Tool {
    fn to_string(&self) -> String {
        let slice = match self {
            Tool::CalculateHp => "Calculate HP",
            Tool::CalculateAc => "Calculate AC",
        };
        slice.to_string()
    }
}

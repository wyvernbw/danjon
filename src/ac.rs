use strum::IntoEnumIterator;

use crate::{
    dnd::{Ac, Armor, Shield, UnarmoredDefense},
    prelude::*,
};

pub(crate) fn calculate_ac() -> Result<(), Box<dyn std::error::Error>> {
    let armor = select("What armor are you wearing?", Armor::iter().collect());
    let dex = input_map("What is your dex modifier?", str::parse::<u8>);
    let unarmored_defense = if let Armor::NoArmor = armor {
        let class = select(
            "Are you a Barbarian or Monk?",
            vec!["Barbarian", "Monk", "Neither"],
        );
        match class {
            "Barbarian" => {
                let con = input_map("What is your Constitution modifier?", str::parse::<u8>);
                UnarmoredDefense::Barbarian(con)
            }
            "Monk" => {
                let wisdom = input_map("What is your Wisdom modifier?", str::parse::<u8>);
                UnarmoredDefense::Monk(wisdom)
            }
            "Neither" => UnarmoredDefense::None,
            _ => unreachable!("Invalid class"),
        }
    } else {
        UnarmoredDefense::None
    };
    let shield = confirm("Are you using a shield?");
    let shield = match shield {
        true => Shield::Shield,
        false => Shield::NoShield,
    };

    let ac = Ac(armor, dex, shield, unarmored_defense);
    let result = ac.calculate();

    tracing::info!("Your AC is {}", result);
    if let (Shield::Shield, UnarmoredDefense::Monk(ability)) = (shield, unarmored_defense) {
        tracing::info!("Tip: Monks lose their Unarmored Defense when using a shield");
        if ability > 2 {
            tracing::info!("You are losing out on {} AC by using a shield", ability - 2)
        }
    }

    Ok(())
}

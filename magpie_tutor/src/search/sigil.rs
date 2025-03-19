use poise::serenity_prelude::colours::roles;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};

use crate::{fuzzy_best, FuzzyRes, Set};

/// Search for a sigil using a set and a name
pub fn sigil_search(set: &Set, sigil_name: &str) -> CreateEmbed {
    let Some(FuzzyRes { rank, data: name }) = fuzzy_best(
        sigil_name,
        set.sigils_description.keys().collect(),
        0.5,
        |s: &String| s.as_str(),
    ) else {
        return CreateEmbed::new()
            .color(roles::RED)
            .title(format!("Sigil \"{sigil_name}\" not found"))
            .description(
                "No sigil found with sufficient similarity with the search term in the selected set(s).",
            );
    };

    CreateEmbed::new()
        .color(roles::TEAL)
        .title(name)
        .description(set.sigils_description.get(name).unwrap())
        .footer(CreateEmbedFooter::new(format!(
            "Match {:.2}% with the search term",
            rank * 100.
        )))
}

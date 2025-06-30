//! Contain implementation for generate card embed from card and a few other info
use poise::serenity_prelude::{colours::roles, CreateEmbed, CreateEmbedFooter};

use crate::{
    emojis::{number, ToEmoji},
    Card, Set,
};

mod aug;
mod desc;
mod imf;

type EmbedRes = (CreateEmbed, String);

/// Generate card embed from a card data.
///
/// The name of the card is store in the embed title along with the set name and any trais flags
/// icon.
///
/// General info like cost, stats, desciption are store inside the embed description because they
/// will always be there
///
/// Sigils and other traits use the embed field because they are optional and not every card have
/// them.
pub fn gen_embed(
    rank: f32,
    card: &Card,
    set: &Set,
    compact: bool,
    unused_mod: String,
) -> CreateEmbed {
    // The specific gen embed function should return the embed and the footer that they would like
    // to add.

    let (embed, footer) = match card.set.code() {
        "aug" | "Aug" | "cti" => aug::gen_embed(card, set, compact),
        "std" | "ete" | "egg" => imf::gen_embed(card, set, compact),
        "des" => desc::gen_embed(card, set, compact),
        code => (
            CreateEmbed::new().color(roles::RED).description(format!(
                "Embed generator for this set code have not been implemented: {code}"
            )),
            String::new(),
        ),
    };

    let footer = {
        let t = format!("{footer}\nMatch {:.2}% with the search term", rank * 100.);
        if unused_mod.is_empty() {
            t
        } else {
            format!(
                "{t}\nThe following modifier was ignored: {}",
                unused_mod
                    .chars()
                    .map(String::from)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    };

    embed.footer(CreateEmbedFooter::new(footer))
}

#[allow(clippy::inline_always)] // this is just a helper function so inline it
#[inline(always)]
fn append_cost(out: &mut String, count: isize, labe: &str, icon: &str) {
    #[rustfmt::skip] // it look nicer like this
    let t = format!( "**{} Cost:**{}{}{}\n", labe, icon, number::X, count.to_emoji());

    if count != 0 {
        out.push_str(&t);
    }
}

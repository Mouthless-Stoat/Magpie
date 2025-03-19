//! Implementation for [IMF] set.
//!
//! [IMF]: https://107zxz.itch.io/inscryption-multiplayer-godot

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    fetch::fetch_json, helper::FlagsExt, Attack, Card, Costs, Mox, Rarity, Set, SetCode, SpAtk,
    Temple, Traits, TraitsFlag,
};

use super::{SetError, SetResult};

/// Fetch a IMF Set from a url.
pub fn fetch_imf_set(url: &str, code: SetCode) -> SetResult<(), ()> {
    let set: ImfSet = fetch_json(url).map_err(|e| SetError::FetchError(e, url.to_string()))?;

    let mut cards = Vec::with_capacity(set.cards.len() + 1);

    let mut sigils_description = HashMap::with_capacity(set.sigils.len());

    for s in set.sigils {
        sigils_description.insert(s.0, s.1);
    }

    sigils_description.insert(
        String::from("UNDEFINEDED SIGILS"),
        "THIS SIGIL IS NOT DEFINED BY THE SET".to_owned(),
    );

    for c in set.cards {
        let card = Card {
            set: code,

            portrait: c
                .pixport_url
                .is_empty()
                .then_some(format!(
                    "https://raw.githubusercontent.com/107zxz/inscr-onln/main/gfx/pixport/{}.png",
                    c.name.replace(' ', "%20")
                ))
                .unwrap_or(c.pixport_url),

            name: c.name,
            description: c.description,

            rarity: if c.rare { Rarity::RARE } else { Rarity::COMMON },
            temple: Temple::empty()
                .set_if(Temple::BEAST, c.blood_cost != 0)
                .set_if(Temple::UNDEAD, c.bone_cost != 0)
                .set_if(Temple::TECH, c.energy_cost != 0)
                .set_if(Temple::MAGICK, !c.mox_cost.is_empty()),
            tribes: None,

            attack: {
                if c.atkspecial.is_empty() {
                    Attack::Num(c.attack)
                } else {
                    let atk = c.atkspecial.as_str();
                    Attack::SpAtk(match atk {
                        "mox" => SpAtk::MOX,
                        "green_mox" => SpAtk::GREEN_MOX,
                        "mirror" => SpAtk::MIRROR,
                        "ant" => SpAtk::ANT,
                        "Bell" => SpAtk::BELL,
                        "Hand" => SpAtk::CARD,
                        _ => return Err(SetError::UnknownSpAtk(c.atkspecial)),
                    })
                }
            },
            health: c.health,
            sigils: c
                .sigils
                .into_iter()
                .map(|s| {
                    if sigils_description.contains_key(&s) {
                        s
                    } else {
                        String::from("UNDEFINEDED SIGILS")
                    }
                })
                .collect(),

            costs: ((c.blood_cost > 0)
                | (c.bone_cost > 0)
                | (c.energy_cost > 0)
                | (!c.mox_cost.is_empty()))
            .then(|| Costs {
                blood: c.blood_cost,
                bone: c.bone_cost,
                energy: c.energy_cost,
                mox: c
                    .mox_cost
                    .iter()
                    .fold(Mox::empty(), |flags, mox| match mox.as_str() {
                        "Orange" => flags | Mox::O,
                        "Green" => flags | Mox::G,
                        "Blue" => flags | Mox::B,
                        _ => unreachable!(),
                    }),
                mox_count: None,
                extra: (),
            }),

            traits: (c.conduit | c.banned | c.nosac | c.nohammer).then(|| Traits {
                strings: None,
                flags: TraitsFlag::empty()
                    .set_if(TraitsFlag::CONDUCTIVE, c.conduit)
                    .set_if(TraitsFlag::BAN, c.banned)
                    .set_if(TraitsFlag::TERRAIN, c.nosac)
                    .set_if(TraitsFlag::HARD, c.nohammer),
            }),

            related: {
                let mut v = Vec::new();

                if !c.evolution.is_empty() {
                    v.push(c.evolution);
                }

                if !c.left_half.is_empty() {
                    v.push(c.left_half);
                }

                if !c.right_half.is_empty() {
                    v.push(c.right_half);
                }

                v
            },

            extra: (),
        };

        cards.push(card);
    }
    Ok(Set {
        code,
        name: set.ruleset,
        cards,
        sigils_description,
    })
}

/// Json scheme for IMF set.
#[derive(Serialize, Deserialize, Debug)]
struct ImfSet {
    ruleset: String,
    cards: Vec<ImfCard>,
    sigils: HashMap<String, String>,
}

/// Json scheme for IMF card.
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
struct ImfCard {
    pub name: String,

    #[serde(default)]
    pub description: String,

    pub attack: isize,
    pub health: isize,

    #[serde(default)]
    pub sigils: Vec<String>,

    #[serde(default)]
    pub atkspecial: String,

    #[serde(default)]
    pub blood_cost: isize,
    #[serde(default)]
    pub bone_cost: isize,
    #[serde(default)]
    pub energy_cost: isize,
    #[serde(default)]
    pub mox_cost: Vec<String>,

    #[serde(default)]
    pub pixport_url: String,

    #[serde(default)]
    pub conduit: bool,
    #[serde(default)]
    pub banned: bool,
    #[serde(default)]
    pub rare: bool,
    #[serde(default)]
    pub nosac: bool,
    #[serde(default)]
    pub nohammer: bool,

    #[serde(default)]
    pub evolution: String,
    #[serde(default)]
    pub left_half: String,
    #[serde(default)]
    pub right_half: String,
}

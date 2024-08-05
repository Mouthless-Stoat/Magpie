//! Implementation for the [Custom TCG Inscryption] set
//!
//! [Custom TCG Inscryption]: https://www.notion.so/inscryption-pvp-wiki/Custom-TCG-Inscryption-3f22fc55858d4cfab2061783b5120f87

use super::{fetch_json, FetchError};
use crate::Rarity;
use crate::{self_upgrade, Card, Costs, Mox, MoxCount, Set, SetCode, Temple, Traits};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

/// Ctimented's [`Card`] extensions
#[derive(Debug, Default, Clone)]
pub struct CtiExt {
    /// Shattered mox cost count.
    pub shattered_count: Option<MoxCount>,
    /// Max energy cell cost.
    pub max: isize,
    // Skulls Cost
    pub skull: isize,
}

self_upgrade!(CtiExt);

/// Fetch Custom TCG Inscryption from the
/// [sheet](https://docs.google.com/spreadsheets/d/152SuTx1fVc4zsqL4_zVDPx69sd9vYWikc2Ce9Y5vhJE/edit?gid=0#gid=0).
#[allow(clippy::too_many_lines)]
pub fn fetch_cti_set(code: SetCode) -> Result<Set<CtiExt>, CtiError> {
    let raw_card: Vec<CtiCard> =
        fetch_json("https://opensheet.elk.sh/152SuTx1fVc4zsqL4_zVDPx69sd9vYWikc2Ce9Y5vhJE/1")
            .map_err(CtiError::CardFetchError)?;

    let sigil: Vec<CtiSigil> =
        fetch_json("https://opensheet.elk.sh/152SuTx1fVc4zsqL4_zVDPx69sd9vYWikc2Ce9Y5vhJE/2")
            .map_err(CtiError::SigilFetchError)?;

    let mut cards = Vec::with_capacity(raw_card.len());

    let undefined_sigil = String::from("UNDEFINDED SIGILS");

    let mut sigils_description = HashMap::with_capacity(sigil.len());

    for s in sigil {
        sigils_description.insert(s.name, s.text.replace('\n', ""));
    }

    sigils_description.insert(
        undefined_sigil.clone(),
        "THIS SIGIL IS NOT DEFINED BY THE SET".to_owned(),
    );

    for card in raw_card {
        let costs;

        let mut shattered_count = MoxCount::default();
        let mut mox_count = MoxCount::default();
        let mut max = 0;
        let mut skull = 0;

        if card.cost != "free" && !card.cost.is_empty() {
            let mut t = Costs::default();

            for c in card
                .cost
                .replace("bones", "bone")
                .replace("rubies", "ruby")
                .replace("emeralds", "emerald")
                .replace("sapphires", "sapphire")
                .replace("prisms", "prism")
                .replace("skulls", "skull")
                .replace("amethysts", "amethyst")
                .replace("topazes", "topaz")
                .replace("garnets", "garnet")
                .split('+')
            {
                let (count, mut cost): (isize, Vec<String>) = {
                    let s = c.to_lowercase().trim().to_string();
                    let mut t = s.split_whitespace().map(ToOwned::to_owned);

                    let first = t
                        .next()
                        .ok_or_else(|| CtiError::InvalidCostFormat(card.cost.clone()))?
                        .parse::<isize>()
                        .map_err(|_| CtiError::InvalidCostFormat(card.cost.clone()))?;
                    let mut rest = t.collect::<Vec<String>>();

                    rest.reverse();
                    (first, rest)
                };

                match cost
                    .pop()
                    .ok_or_else(|| CtiError::InvalidCostFormat(card.cost.clone()))?
                    .as_str()
                {
                    "blood" => t.blood += count,
                    "bone" => t.bone += count,
                    "energy" => t.energy += count,
                    "max" => max += count,
                    "skull" => skull += count,
                    "shattered" => match cost.pop().unwrap().as_str() {
                        "ruby" => {
                            t.mox |= Mox::R;
                            shattered_count.r += count as usize;
                        }
                        "emerald" => {
                            t.mox |= Mox::G;
                            shattered_count.g += count as usize;
                        }
                        "sapphire" => {
                            t.mox |= Mox::B;
                            shattered_count.b += count as usize;
                        }
                        "prism" => {
                            t.mox |= Mox::Y;
                            shattered_count.y += count as usize;
                        }
                        "topaz" => {
                            t.mox |= Mox::T;
                            shattered_count.t += count as usize;
                        }
                        "amethyst" => {
                            t.mox |= Mox::P;
                            shattered_count.p += count as usize;
                        }
                        "garnet" => {
                            t.mox |= Mox::O;
                            shattered_count.o += count as usize;
                        }
                        m => return Err(CtiError::UnknowMox(m.to_owned())),
                    },
                    m @ ("ruby" | "sapphire" | "emerald" | "prism" | "topaz" | "amethyst" | "garnet") => match m {
                        "ruby" => {
                            t.mox |= Mox::R;
                            mox_count.r += count as usize;
                        }
                        "emerald" => {
                            t.mox |= Mox::G;
                            mox_count.g += count as usize;
                        }
                        "sapphire" => {
                            t.mox |= Mox::B;
                            mox_count.b += count as usize;
                        }
                        "prism" => {
                            t.mox |= Mox::Y;
                            mox_count.y += count as usize;
                        }
                        "topaz" => {
                            t.mox |= Mox::T;
                            mox_count.t += count as usize;
                        }
                        "amethyst" => {
                            t.mox |= Mox::P;
                            mox_count.p += count as usize;
                        }
                        "garnet" => {
                            t.mox |= Mox::O;
                            mox_count.o += count as usize;
                        }
                        _ => unreachable!(),
                    },
                    "asterisk" => (),
                    c => return Err(CtiError::UnknowCost(c.to_string())),
                }
            }
            if mox_count != MoxCount::default() {
                t.mox_count = Some(mox_count);
            }
            costs = Some(t);
        } else {
            costs = None;
        }

        let card = Card {
            portrait: card.portrait,
            set: code,

            name: card.name,
            description: card.description,

            rarity: match card.rarity.as_str() {
                "Common" | "" => Rarity::COMMON,
                "Uncommon" => Rarity::UNCOMMON,
                "Rare" => Rarity::RARE,
                "Talking" => Rarity::UNIQUE,
                "Side Deck" => Rarity::SIDE,
                "Deathcard" => Rarity::DEATHCARD,
                "Common (Joke Card)" => Rarity::JOKECARD,
                _ => return Err(CtiError::UnknownRarity(card.rarity)),
            },
            temple:match card.temple.as_str() {
                "Beast" => Temple::BEAST,
                "Undead" => Temple::UNDEAD,
                "Tech" => Temple::TECH,
                "Magick" => Temple::MAGICK,
                "Fool" => Temple::FOOL,
                "Terrain/Extras" => Temple::EXTRAS,
                _ => return Err(CtiError::UnknownTemple(card.temple))
            }.into(),
            tribes: None,

            attack: card.attack.parse().unwrap_or(0),
            health: card.health.parse().unwrap_or(0),
            sigils:
                sigilslist = [card.sigil1, card.sigil2, card.sigil3, card.sigil4].to_vec(),
                sigilslist.split(", ").map(|s| {
                    let s = s.to_owned();
                    if sigils_description.contains_key(&s) {
                        s
                    } else {
                        String::from("UNDEFINEDED SIGILS")
                    }
                }).collect(),
            // I don't pay enough attention to Ctimented to keep updating the code to accommodate
            // them so the value will just be parse as string
            sp_atk: None,

            costs,

            traits: None,
            related: if card.token.is_empty() {
                vec![]
            } else {
                card.token.split(", ").map(ToOwned::to_owned).collect()
            },

            extra: CtiExt {
                max,
                shattered_count: (!shattered_count.eq(&MoxCount::default())).then_some(shattered_count),
                skull,
            },

        };

        cards.push(card);
    }

    Ok(Set {
        code,
        name: String::from("Custom TCG Inscryption"),
        cards,
        sigils_description,
    })
}

/// Error that happen when calling [`fetch_Cti_set`].
#[derive(Debug)]
pub enum CtiError {
    /// Error when trying to [`fetch_json`] cards.
    CardFetchError(FetchError),
    /// Error when trying to [`fetch_json`] sigils.
    SigilFetchError(FetchError),
    /// Invalid Rarity.
    UnknownRarity(String),
    /// Invalid Temple.
    UnknownTemple(String),
    /// Invalid cost format. The cost doesn't follow each component are a number then the cost
    /// with space between and every cost is separted by `'+'`.
    InvalidCostFormat(String),
    /// Unknow cost.
    UnknowCost(String),
    /// Invalid Mox color.
    UnknowMox(String),
}

impl Display for CtiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtiError::CardFetchError(e) => write!(f, "cannot fetch cards due to: {e}"),
            CtiError::SigilFetchError(e) => write!(f, "cannot fetch sigils due to: {e}"),
            CtiError::UnknownRarity(r) => write!(f, "unknown rarity: {r}"),
            CtiError::UnknownTemple(r) => write!(f, "unknown temple: {r}"),
            CtiError::InvalidCostFormat(s) => write!(f, "invalid cost: {s}"),
            CtiError::UnknowCost(c) => write!(f, "unknow cost: {c}"),
            CtiError::UnknowMox(m) => write!(f, "unknow mox: {m}"),
        }
    }
}

/// Json scheme for Cti card
#[derive(Deserialize)]
struct CtiCard {
    #[serde(rename = "Internal Name")]
    internalname: String,
    #[serde(rename = "From")]
    format: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Temple")]
    temple: String,
    #[serde(rename = "Rarity")]
    rarity: String,
    #[serde(rename = "Cost")]
    cost: String,
    #[serde(rename = "Power")]
    attack: String,
    #[serde(rename = "Health")]
    health: String,
    #[serde(rename = "Flavor Text")]
    description: String,
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "Sigil 1")]
    sigil1: String,
    #[serde(rename = "Sigil 2")]
    sigil2: String,
    #[serde(rename = "Sigil 3")]
    sigil3: String,
    #[serde(rename = "Sigil 4")]
    sigil4: String,
    #[serde(rename = "Image")]
    portrait: String,
    #[serde(rename = "Wiki-Page")]
    wikipage: String,
}

/// Json scheme for Cti sigil
#[derive(Deserialize)]
struct CtiSigil {
    #[serde(rename = "Internal Name")]
    internalname: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    text: String,
    #[serde(rename = "Category")]
    category: String,
}

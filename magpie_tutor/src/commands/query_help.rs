use crate::{CmdCtx, Res};

macro_rules! query_help {
    ($($key:ident [$($alias:ident),*]: $desc:literal);*) => {
        concat!(
            "Possible query keyword for searching:\nHow to read: [keyword name] [keyword alias]: [keyword description]\n\n",
            $(concat!("- ", stringify!($key), " [", $(stringify!($alias))*, "]: ", $desc, "\n")),*
        )
    };
}

/// Show a list of all possible query keyword.
#[poise::command(slash_command)]
pub async fn query_help(ctx: CmdCtx<'_>) -> Res {
    ctx.say(query_help!(
        name [n]: "Filter for name.";
        description [d]: "Filter for description.";
        rarity [r]: "Filter for rarity. Possible values: `side`, `common`, `rare`, `unique` as well as their shorthand.";
        temple [tp]: "Filter for temple. Possible values: `beast`, `undead`, `technology`, `fool`, `artistry` as well as their shorthand.";
        tribe [tb]: "Filter for tribe.";
        attack [a]: "Filter for attack.";
        health [h]: "Filter for health.";
        sigil [s]: "Filter for sigil.";
        spatk [sp]: "Filter for special attack. Possible values: `mox`, `green`, `mirror`, `ant`, `bone`, `bell`, `card`.";
        cost [c]: "Filter for cost. Must follow the format of `[amount][type]`. Example `1b` would search for 1 blood, `3b6e2o` would search for 3 blood, 6 energy and 2 bones.";
        costtype [ct]: "Filter for cost type. Possible values: `b`, `o`, `e`, `m` and any combination of them.";
        trait [tr]: "Filter for trait."
    ))
    .await?;

    Ok(())
}

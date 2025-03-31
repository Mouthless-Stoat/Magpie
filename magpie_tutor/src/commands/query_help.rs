use crate::{CmdCtx, Res};

macro_rules! query_help {
    ($($key:ident [$($alias:ident),*]: $desc:literal);*) => {
        concat!(
            "Possible query keyword for searching:\nHow to read: [keyword name] [keyword alias]: [keyword description]\n\n",
            $(concat!(stringify!($key), " [", $(stringify!($alias))*, "]: ", $desc, "\n")),*
        )
    };
}

#[poise::command(slash_command)]
pub async fn show_query(ctx: CmdCtx<'_>) -> Res {
    ctx.say(query_help!(
        name [n]: "Filter for name";
        description [d]: "Filter for description";
        rarity [r]: "Filter for rarity";
        temple [tp]: "Filter for temple";
        tribe [tb]: "Filter for tribe";
        attack [a]: "Filter for attack";
        health [h]: "Filter for health";
        sigil [s]: "Filter for sigil";
        spatk [sp]: "Filter for special attack";
        cost [c]: "Filter for cost";
        costtype [ct]: "Filter for cost type";
        trait [tr]: "Filter for cost"
    ))
    .await?;

    Ok(())
}

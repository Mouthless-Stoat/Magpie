#![allow(missing_docs)]

use crate::{CmdCtx, Res, PING_RESPONSE};
use rand::seq::SliceRandom;
use rand::thread_rng;

mod query_help;
pub use query_help::*;

/// Test command
#[poise::command(slash_command)]
pub async fn test(ctx: CmdCtx<'_>) -> Res {
    ctx.say("Testing").await?;
    Ok(())
}

/// Show help on what and how to use Magpie Tutor.
#[poise::command(slash_command)]
pub async fn help(ctx: CmdCtx<'_>) -> Res {
    ctx.say(
r#"
You can use Magpie to look up a card infomation by surrounding the card name in `[[]]`. A few "modifiers" can be added in front of the `[[]]` to change the output.

You can see these modifier by using the `/show-modifers` command. Set code are a special type of modifer that are 3 characters long and is at the end of the modifiers list and can be use to change the selected set.

For example:
- `[[stoat]]`: Look up the card name `stoat` using the server default set.
- `egg[[warren]]`: Look up the card name `warren` using the `egg` set.

"#,
    )
    .await?;

    Ok(())
}

macro_rules! mod_help {
    ($($code:ident: $code_desc:literal;)*---$($mod:literal: $desc:literal;)*) => {
        concat!(
            "# Set Codes\n",
            $(concat!("- `", stringify!($code), "`: ", $code_desc, ".\n"),)*
            "# Modifiers\n",
            $(concat!("- `", $mod,"`: ", $desc, " ", ".\n"),)*
        )
    };
}

/// Show the lists of all support modifiers and set code.
#[poise::command(slash_command)]
pub async fn show_modifiers(ctx: CmdCtx<'_>) -> Res {
    ctx.say(mod_help! {
        com: "IMF Competitive";
        egg: "Mr.Egg's Goofy";
        ete: "IMF Eternal";
        aug: "Augmented Snapshot";
        Aug: "Augmented Stable";
        des: "Descryption";
        cti: "Custom TCG";
        ---
        "q": "Query instead of normal fuzzy search";
        "*": "Select all supported set";
        "d": "Output the raw data instead of embed";
        "c": "Output the embed in compact mode to save space";
        "s": "Search for sigils instead of cards";
        "r": "Disallow smart detection for query";
        "\\`": "Skip this search match";

    })
    .await?;

    Ok(())
}

/// Test to see if the IMF tunnel is online
#[poise::command(slash_command)]
pub async fn tunnel_status(ctx: CmdCtx<'_>) -> Res {
    ctx.defer().await?;
    ctx.say(match isahc::get("http://localtunnel.me") {
        Ok(_) => "Tunnel is up and running. If you have issue check out [this faq](https://discord.com/channels/994573431880286289/1168644586319659100/1168657617141366805).",
        Err(_) => "I cannot reach tunnel right now, this may mean tunnel is down but you can [check yourself](https://isitdownorjust.me/localtunnel-me/)."
    })
    .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn ping(ctx: CmdCtx<'_>) -> Res {
    let choose = PING_RESPONSE.choose(&mut thread_rng());
    ctx.say(*choose.unwrap()).await?;
    Ok(())
}

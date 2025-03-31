#![allow(missing_docs)]

use std::panic::PanicHookInfo;

use magpie_tutor::{
    commands::*, done, error, frameworks, info, Color, Data, CACHE, CACHE_FILE_PATH, SETS,
};
use poise::serenity_prelude::{CacheHttp, ClientBuilder, GatewayIntents, GuildId};

// main entry point of the bot
#[tokio::main]
async fn main() {
    // your token need to be in the enviroment variable
    let token = std::env::var("TUTOR_TOKEN").expect("missing token in env var");
    let intents = GatewayIntents::privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // poise framework
    let framework = frameworks! {
        global: help(), show_modifiers(), ping(), show_query();
        guild (1199457939333849118): test();
        guild (994573431880286289): tunnel_status();
        ---
        {
            Ok(Data::new())
        }
    };

    info!("Fetching set...");
    done!(
        "Finish fetching {} sets",
        SETS.lock().unwrap().len().green()
    );

    info!("Loading caches from {}...", CACHE_FILE_PATH.green());
    done!(
        "Finish loading {} caches",
        CACHE.lock().unwrap().len().green()
    );

    std::panic::set_hook(Box::new(panic_hook));

    // client time
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

fn panic_hook(info: &PanicHookInfo) {
    if let Some(loc) = info.location() {
        error!(
            "Panic in file {} at line {}",
            loc.file().magenta(),
            loc.line().blue()
        );
    }

    let s = info
        .payload()
        .downcast_ref::<String>()
        .map(ToOwned::to_owned)
        .or_else(|| {
            info.payload()
                .downcast_ref::<&str>()
                .map(ToString::to_string)
        })
        .unwrap_or(String::new());

    let lines: Vec<_> = s.lines().collect();
    if lines.len() > 1 {
        error!("Panic message:");
        for l in lines {
            error!("{}", l.red());
        }
    } else {
        error!("Panic message: {}", s.red());
    }
}

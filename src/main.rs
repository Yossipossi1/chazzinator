use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use std::time::Duration;

mod config;
mod database;
mod listeners;
mod poise_ext;

use crate::poise_ext::{Context, Data, Error};

// Registers the commans in Discord.
#[poise::command(prefix_command, owners_only, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, true).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Load environment variables into scope.
    dotenv().ok();
    // Initialize the database.
    database::initialize_database().expect("Unable to initialize database");
    // Build the framework for the bot.
    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![register()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("-".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
                ignore_bots: true,
                ..Default::default()
            },
            listener: |ctx, event, framework, user_data| {
                Box::pin(listeners::event_listeners(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .token(std::env::var("TOKEN").expect("Unable to find token"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::privileged(),
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    // Run the bot.
    framework.run().await.unwrap();
}

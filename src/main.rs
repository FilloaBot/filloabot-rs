mod commands;
mod utils;

use std::env;

use songbird::SerenityInit;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await.expect("Error while responding to slash command");

            let embed = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command),
                "join" => commands::music::join::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                "leave" => commands::music::leave::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                "play" => commands::music::play::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                "pause" => commands::music::pause::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                "resume" => commands::music::resume::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                "queue" => commands::music::queue::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                "next" => commands::music::next::run(&command, &ctx, command.member.as_ref().unwrap()).await,
                _ => todo!(),
            };
            command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.add_embed(embed)
                })
                .await.expect("Error while editing response to slash command");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        utils::chatbot::search_references(&ctx, &msg).await.expect("Error while scanning message");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::music::join::register(command))
                    .create_application_command(|command| commands::music::leave::register(command))
                    .create_application_command(|command| commands::music::play::register(command))
                    .create_application_command(|command| commands::music::pause::register(command))
                    .create_application_command(|command| commands::music::resume::register(command))
                    .create_application_command(|command| commands::music::next::register(command))
                    .create_application_command(|command| commands::music::queue::register(command))
            })
            .await.expect("Error adding the application commands");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
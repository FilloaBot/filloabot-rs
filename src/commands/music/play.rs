use super::join;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(options: &[CommandDataOption], ctx: &Context, member: &Member) -> String {
    let option = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let query = if let CommandDataOptionValue::String(query) = option { query.as_str() } else { "" };

    let output = join::run(options, &ctx, &member).await;
    if output != "Connected!" {
        return output
    }

    let guild_id = member.guild_id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::input::ytdl_search(&query).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                return "An error occurred while trying to play that song".to_string()
            },
        };

        let (audio, _audio_handle) = songbird::tracks::create_player(source);

        handler.enqueue(audio);

        return "Playing song".to_string()
    } else {
        return "Not in a voice channel to play in".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Play some music").create_option(|option| {
        option
            .name("query")
            .description("URL or search string to play")
            .kind(CommandOptionType::String)
            .required(true)
    })
}
use super::join;
use super::resume;

use serenity::utils::Colour;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOptionValue, ApplicationCommandInteraction};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, member: &Member) -> CreateEmbed {
    let mut embed: CreateEmbed = Default::default();

    let option = match command.data.options.get(0) {
        Some(option) => option.resolved.as_ref().expect("Expected string object"),
        None => {
            return resume::run(&command, &ctx, &member).await;
        },
    };

    let query = if let CommandDataOptionValue::String(query) = option { query.as_str() } else { "" };

    let _output = join::run(&command, &ctx, &member).await;

    let guild_id = member.guild_id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::input::ytdl_search(&query).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                return embed.colour(Colour::DARK_RED).title("No results").description(format!("There aren't any results for **{}**", query)).clone()
            },
        };

        let metadata = source.metadata.clone();
        let (audio, _audio_handle) = songbird::tracks::create_player(source);

        handler.enqueue(audio);

        if handler.queue().len() == 1 {
            embed.title("Playing song");
        } else {
            embed.title("Added song to queue");
        }
        return embed.colour(Colour::DARK_BLUE).description(format!("[{}]({})", metadata.title.unwrap_or_default(), metadata.source_url.unwrap_or_default())).clone()
    } else {
        return embed.colour(Colour::DARK_RED).title("Not in a voice channel").clone()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Play some music").create_option(|option| {
        option
            .name("query")
            .description("URL or search string to play. Without arguments it works as /resume")
            .kind(CommandOptionType::String)
    })
}
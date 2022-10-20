use serenity::utils::Colour;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(_command: &ApplicationCommandInteraction, ctx: &Context, member: &Member) -> CreateEmbed {
    let mut embed: CreateEmbed = Default::default();

    let guild_id = member.guild_id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();

        if queue.len() > 1{
            let metadata = queue.current_queue()[1].metadata().clone();
            embed.title("Skipped").field("Now playing", format!("[{}]({})", metadata.title.unwrap_or_default(), metadata.source_url.unwrap_or_default()), false);
        } else {
            embed.title("Queue is now empty");
        }
        
        queue.skip().expect("Error skipping current track");

        return embed.colour(Colour::DARK_BLUE).clone()
    } else {
        return embed.colour(Colour::DARK_RED).title("Not in a voice channel").clone()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("next").description("Skips to the next track in the queue")
}
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

        let queue = handler.queue().current_queue();

        if queue.len() >= 1 {
            for (i, track) in queue.into_iter().enumerate() {
                let metadata = track.metadata().clone();
                embed.field(i, format!("[{}]({})", metadata.title.unwrap_or_default(), metadata.source_url.unwrap_or_default()), false);
            }
        } else {
            embed.description("The queue is empty");
        }

        return embed.colour(Colour::DARK_BLUE).title("Queue").clone()
    } else {
        return embed.colour(Colour::DARK_RED).title("Not in a voice channel").clone()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("queue").description("Shows the track queue")
}
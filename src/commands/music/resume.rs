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

        handler.queue().resume().expect("Error resuming the queue");

        return embed.colour(Colour::DARK_BLUE).title("Resumed playback").clone()
    } else {
        return embed.colour(Colour::DARK_RED).title("Not in a voice channel").clone()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("resume").description("Resume the current track")
}
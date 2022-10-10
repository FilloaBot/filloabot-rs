use serenity::utils::Colour;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(_command: &ApplicationCommandInteraction, ctx: &Context, member: &Member) -> CreateEmbed {
    let mut embed: CreateEmbed = Default::default();

    let guild_id = member.guild_id;
    let guild = ctx.cache.guild(guild_id).unwrap();

    let channel_id = guild
        .voice_states.get(&member.user.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return embed.colour(Colour::DARK_RED).title("Not in a voice channel").clone()
        }
    };

    let manager = songbird::get(&ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(_e) = handler.deafen(true).await { 
            return embed.colour(Colour::DARK_RED).title("There was an error").clone()
        }
    } else {
        return embed.colour(Colour::DARK_RED).title("There was an error").clone()
    }

    return embed.colour(Colour::DARK_BLUE).title("Connected!").clone()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("Join the voice channel you're in")
}
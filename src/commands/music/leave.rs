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
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(_e) = manager.remove(guild_id).await {
            return embed.colour(Colour::DARK_RED).title("There was an error").clone()
        } else {
            return embed.colour(Colour::DARK_BLUE).title("Left the voice channel").clone()
        }
    } else {
        return embed.colour(Colour::DARK_RED).title("Not in a voice channel").clone()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("Leave the voice channel the bot is in")
}
use serenity::utils::Colour;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn run(_command: &ApplicationCommandInteraction) -> CreateEmbed {
    let mut embed: CreateEmbed = Default::default();
    return embed.colour(Colour::DARK_BLUE).title("Pong!").clone()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
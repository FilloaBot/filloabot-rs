use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, member: &Member) -> String {
    let guild_id = member.guild_id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(_e) = manager.remove(guild_id).await {
            return "There was an error".to_string() 
        } else {
            return "Left voice channel".to_string()
        }
    } else {
        return "Not in a voice channel".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("Leave the voice channel the bot is in")
}
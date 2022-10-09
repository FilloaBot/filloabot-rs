use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, member: &Member) -> String {
    let guild_id = member.guild_id;
    let guild = ctx.cache.guild(guild_id).unwrap();

    let channel_id = guild
        .voice_states.get(&member.user.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return "Not in a voice channel".to_string()
        }
    };

    let manager = songbird::get(&ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(_e) = handler.deafen(true).await { 
            return "There was an error".to_string() 
        }
    } else {
        return "There was an error".to_string() 
    }

    return "Connected!".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("Join the voice channel you're in")
}
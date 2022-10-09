use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, member: &Member) -> String {
    let guild_id = member.guild_id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let audio_handle = handler.queue().current().expect("Error retrieving current track's hanlde");
        audio_handle.stop().expect("Error while stopping track");

        return "Stopped".to_string()
    } else {
        return "Not in a voice channel".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stop").description("Stop the music")
}
use super::api;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct Reference {
    #[serde(with = "serde_regex")]
    regex: Regex,
    message_reactions: Vec<char>,
    answer_text: String,
    answer_image: String,
    answer_reactions: Vec<char>,
}

#[derive(Serialize, Deserialize)]
struct References {
    references: Vec<Reference>,
}

pub async fn search_references(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    let references = api::references::get(*msg.guild_id.expect("Message wasn't from guild").as_u64()).await;

    for reference in references.iter() {
        if reference.regex.is_match(msg.content.as_str()) {
            for reaction in reference.message_reactions.iter() {
                msg.react(ctx, ReactionType::from(*reaction)).await?;
            }

            if !reference.answer_image.is_empty() || !reference.answer_text.is_empty() {
                let answer_msg = msg.channel_id.send_message(ctx, |m| {
                    m.content(reference.answer_text.as_str())
                    .reference_message(msg)
                    .allowed_mentions(|am| am.empty_parse());
                    if !reference.answer_image.is_empty() {
                        m.add_file(reference.answer_image.as_str());
                    }
                    m
                }).await?;
                for reaction in reference.answer_reactions.iter() {
                    answer_msg.react(ctx, ReactionType::from(*reaction)).await?;
                }
            }
        }
    }

    Ok(())
}
// use serenity::framework::standard::macros::hook;
use serenity::model::prelude::*;
use serenity::prelude::*;
// use serenity::Result;
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
    let json = r#"
    {"references": [
        {"regex": "cepill",
        "message_reactions": ["🪥"],
        "answer_text": "Cuidadito",
        "answer_image": "https://cdn.discordapp.com/attachments/944942658713952317/1026547050038444032/cepill_II9iw.png",
        "answer_reactions": ["😎"]}
    ]}
"#;


    let parsed: References = serde_json::from_str(json).unwrap();
    let references = parsed.references;

    for reference in references.iter() {
        if reference.regex.is_match(&*msg.content) {
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

// #[hook]
// pub async fn message_analyzer(ctx: &Context, msg: &Message){
//     search_references(&ctx, &msg).await.unwrap();
// }
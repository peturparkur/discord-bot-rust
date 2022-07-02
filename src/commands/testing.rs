
use serde::{Serialize, Deserialize};
// https://api.dictionaryapi.dev/api/v2/entries/en/<word> API for word definitions
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::prelude::*;

#[command]
pub async fn whatup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let _message = msg.channel_id.send_message(&ctx.http, move |m| {
        m.tts(true); // The important part
        m.content("What's up homie!");
        m
    }).await.unwrap();

    return Ok(())
}
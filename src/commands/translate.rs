// https://api.dictionaryapi.dev/api/v2/entries/en/<word> API for word definitions
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let word = args.single::<String>()?;

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let response = reqwest::get(url).await?;

    let json: serde_json::Value = response.json().await?;
    let _doc = json
                                        .as_array();
    let document = match _doc {
        Some(x) => x,
        _ => {
            msg.channel_id.say(&ctx.http, format!("No definition for {}", &word)).await?;
            return Ok(());
        }
    }
    .iter()
    .nth(0)
    .unwrap()
    .as_object()
    .unwrap();
    println!("{:?}", document);

    let word = document.get("word").unwrap().as_str().unwrap();
    println!("{:?}", word);

    // (type, definitions)
    let definitions = document
                                                    .get("meanings")
                                                    .unwrap()
                                                    .as_array()
                                                    .unwrap()
                                                    .iter()
                                                    .map(
                                                        |definition| 
                                                        definition
                                                        .as_object()
                                                        .unwrap()
                                                    )
                                                    .map(
                                                        |def| 
                                                        (
                                                            def
                                                            .get("partOfSpeech")
                                                            .unwrap()
                                                            .as_str()
                                                            .unwrap(),
                                                            def
                                                            .get("definitions")
                                                            .unwrap()
                                                            .as_array()
                                                            .unwrap()
                                                            .iter()
                                                            .map(|d| 
                                                                // String::from(
                                                                d.as_object()
                                                                .unwrap()
                                                                .get("definition")
                                                                .unwrap()
                                                                .as_str()
                                                                .unwrap()
                                                                // )
                                                            )
                                                            .enumerate()
                                                            .fold(
                                                                String::from(""),
                                                                |acc, (idx, _str)| acc + &format!("{}. ", idx) + _str + "\n"
                                                            )
                                                            // .collect::<Vec<&str>>()
                                                            // .join(" \n ")
                                                            // .fold(String::from(""),
                                                            //     |curr, x| curr.concat(x).concat("\n")
                                                            // )
                                                            // .collect::<Vec<&str>>()
                                                        )
                                                    )
                                                    .fold(String::from(""), 
                                                        |curr, (x, y)| curr + x + " \n " + y.as_str() + "------ \n"
                                                    );
                                                    // .collect::<Vec<(&str, String)>>();
    // println!("{:?}", definitions);

    // println!("{:?}", json);
    // println!("--------");
    // println!("{:?}", definition);

    // let _out = format!("{:?} => {:?}", word, definitions);
    // println!{"{:?}", _out};
    msg.channel_id.say(&ctx.http, definitions).await?;

    Ok(())
}
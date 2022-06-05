use reqwest::Url;
use serde::{Serialize, Deserialize};
// https://api.dictionaryapi.dev/api/v2/entries/en/<word> API for word definitions
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

use serde_json::Value;
use std::collections::HashMap;

// dotenv::dotenv().expect("Failed to load .env file");
// let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Post {
    author: String, // Op
    title: String,
    subreddit: String,
    id: String,
    post_hint: Option<String>, // "image", "video", "link", etc... -> Tries to hint on processing required // if post hint is None => Text-post (one example)
    is_video: bool, // video???
    over_18: bool, // adult alert
    score: u32, // score??? upvotes + others - downvotes???
    spoiler: bool, // spoiler alert
    ups: u32, // upvotes
    downs: u32, // downvotes
    upvote_ratio: f64, // Ups / Downs
    url: String, // idk??
    url_overridden_by_dest: Option<String>, // image link
    created_utc: Option<f64>, // time of creation
    self_text: Option<String>, // if text post -> content
}


/// The command should go along the lines of: `<subreddit> <number of posts> <order>`
#[command]
pub async fn reddit(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let subreddit: String = args.single()?;
    let count = args.single::<u32>().unwrap_or(50);
    let order = args.single::<String>().unwrap_or("hot".to_string());

    // let order = "new".to_string(); // Could be hot, new, top
    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, order);
    // println!("url: {}", url);
    let client = reqwest::Client::new();
    let response = client.get(url)
            .header("User-Agent", "Rusty_Discord_Bot/0.0.1")
            .query(&[
                ("limit", count.to_string())]
            )
            .send()
            .await.expect("Failed to send request");

    // println!("{}", response.text().await.expect("Failed to get text"));
    let response_json: serde_json::Value = response.json().await.expect("Failed to get json");
    let obj = response_json
        .as_object()
        .unwrap();
    // println!("{:?}", &obj);

    if obj.get("error").is_some() {
        let error = obj.get("error").unwrap();
        let error_msg = error.as_str().unwrap();
        msg.channel_id.say(&ctx.http, error_msg).await?;
        return Ok(());
    }
    let data = obj
        .get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("children")
        .unwrap()
        .as_array()
        .unwrap();
    println!("{:?}", data.first().unwrap());
    let posts = data
        .iter()
        .map(
            |x| 
            serde_json::from_value::<Post>(
                x.get("data").unwrap().clone()).unwrap()
        )
        .collect::<Vec<Post>>();
    println!("{:?}", posts.first().unwrap());

    posts.iter().for_each(|x| {
        if x.url_overridden_by_dest.is_none() {
            println!("No URL for POST: {:?}", x);
        }
    });

    // posts have a data struct to describe them

    msg.channel_id.send_message(&ctx.http, move |m| {
        let p = posts.first().unwrap();
        if p.post_hint.is_none(){
            m.content("".to_string() + &p.title + &"\n".to_string() + &p.self_text.as_ref().unwrap());
        }
        else{
            m.content(&p.title);
            // let _resp = reqwest::get(p.url_overridden_by_dest.as_ref().unwrap());
            // m.add_embed(
            //     |e| {
            //         e.image(&p.url_overridden_by_dest.as_ref().unwrap());
            //         e
            //     });
            m.add_file(serenity::model::channel::AttachmentType::Image(Url::parse(p.url_overridden_by_dest.as_ref().unwrap()).expect("Couldnt parse url")));
        }
        return m
    }).await?;
    Ok(())
}
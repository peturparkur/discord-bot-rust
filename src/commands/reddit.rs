use reqwest::Url;
use serde::{Serialize, Deserialize};
// https://api.dictionaryapi.dev/api/v2/entries/en/<word> API for word definitions
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::prelude::*;
// use std::env;

use serde_json::Value;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use std::io::Write;

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
    selftext: Option<String>, // if text post -> content
}


/// The command should go along the lines of: `<subreddit> <number of posts> <order>`
#[command]
pub async fn reddit(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let subreddit: String = args.single()?;
    let order = args.single::<String>().unwrap_or("hot".to_string());
    let count = args.single::<u32>().unwrap_or(50);
    let number = match args.single::<i32>() { Ok(x) => Some(x - 1), Err(err) => None}; // -1 indicates we take a random from the entire range

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
    // println!("{:?}", data.first().unwrap());
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

    // select which post we want to send
    let _post = match number {
        Some(x) => {
            posts.iter().nth(std::cmp::max(x, 0) as usize).unwrap()
        },
        None => {
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0..posts.len());

            posts.iter().nth(r).unwrap()
        }
    };

    // get the extension of the file -> this is to get .jpg, .png, .mp4, etc...
    let _extension = match &_post.url_overridden_by_dest
        .as_ref() {
            Some(x) => Some(x
                .split(".")
                .last()
                .unwrap()
            ),
            None => None,
        };
    
    // Getting image content ahead of time -> Due to message contructor is synchronoused code
    let _img_bytes = match &_post.url_overridden_by_dest {
        Some(_url) => {
            Ok(reqwest::get(_url)
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap())
        },
        None => {
            Err("Not Good")
        }
    };
    
    // posts have a data struct to describe them
    let _message = msg.channel_id.send_message(&ctx.http, move |m| {
        let p = _post; // the selected post

        println!("POST: \n{:?}", &p);

        let mut r = (&p.title).clone();

        if p.post_hint.is_none(){
            // Text Content
            r += &match &p.selftext.as_ref() {
                Some(s) => {
                    // Goddamn this is looking weird :P
                    |s : &&String| -> String {
                        if s == &&"".to_string() {
                            return "".to_string()
                        }
                        return format!("\n--------\n{}", s)
                    }(s)
                },
                None => {
                    "".to_string()
                },
            };
            m.content(&r);
        }
        else{
            // Multi-media content
            m.content(&r);
            println!("CONTENT TYPE: {}", &p.post_hint.as_ref().unwrap());
            let cow = std::borrow::Cow::from(_img_bytes.unwrap().to_vec());
            println!("Sending -> tmp.{}", _extension.unwrap());

            if p.over_18 {
                m.add_file(AttachmentType::Bytes { data: cow, filename: format!("SPOILER_tmp.{}", _extension.unwrap()) });
            }
            else {
                m.add_file(AttachmentType::Bytes { data: cow, filename: format!("tmp.{}", _extension.unwrap()) });
            }
        }
        return m
    }).await?;

    Ok(())
}
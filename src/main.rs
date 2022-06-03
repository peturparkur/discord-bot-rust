use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

mod commands;
use commands::math::*;
use commands::translate::*;
use commands::alpaca_stocks::*;

// Structure guide https://github.com/serenity-rs/serenity/blob/current/examples/e06_sample_bot_structure/src/main.rs

#[group]
#[commands(ping, multiply, define, stock)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{:?}", msg.content);
    }
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("token_env: {}", token);


    let args = std::env::args().collect::<Vec<_>>();
    // let token = args.get(1).expect("Please provide a token");
    println!("token:{}", token);

    let framework = StandardFramework::new()
        .configure(
            |c| c.prefix("~")
        ) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    // let token = env::var("DISCORD_TOKEN").expect("Token was not given");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    println!("{:?}", intents);
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("msg.channel_id: {:?}", msg.channel_id);
    msg.reply(ctx, "Pong!").await?;

    let member = msg.member(ctx.http.clone()).await.expect("no member");
    println!("member: {:?}", member);

    let guild_name = msg.guild_id.expect("not guild").name(ctx.cache.clone()).expect("no name");
    println!("guild_id: {:?}", guild_name);

    // let activity = msg.activity.clone().expect("no activity");
    // println!("activity: {:?}", activity);

    Ok(())
}
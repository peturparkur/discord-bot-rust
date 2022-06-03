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

// pub fn url(){
//     "GET/v2/stocks/{symbol}/bars"
// }
#[command]
pub async fn stock(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let endpoint = env::var("ALPACA_MARKETDATA_ENDPOINT").expect("Expected a token in the environment");
    let key_id = env::var("ALPACA_KEY_ID").expect("NO alpaca key id");
    let secret_key = env::var("ALPACA_SECRET_KEY").expect("NO alpaca key id");

    let word = args.single::<String>()?;

    // println!("endpoint: {}", endpoint);
    // headers = {}
    // headers['APCA-API-KEY-ID'] = self._key_id
    // headers['APCA-API-SECRET-KEY'] = self.

    let url = format!("{}/stocks/{}/bars/latest", endpoint, word);
    // println!("url: {}", url);
    let client = reqwest::Client::new();
    let response = client.get(url)
            .header("APCA-API-KEY-ID", key_id)
            .header("APCA-API-SECRET-KEY", secret_key)
            .send()
            .await.expect("Failed to send request");
    // let response = reqwest::get(url).await.expect("request failed");

    let json: serde_json::Value = response.json().await?;
    println!("{}", json);

    let dict = json
                                        .as_object()
                                        .unwrap()
                                        .clone();
    let _output:HashMap<String, Value> = dict
                                        .get("bar")
                                        .unwrap()
                                        .as_object()
                                        .unwrap()
                                        .iter()
                                        .map(
                                            |(k, v)| {
                                                // println!("{:?}", k);
                                                // println!("{:?}", v);
                                                // if k == "t"{
                                                //     return (k.clone(), 0.0);
                                                // }
                                                // (k.clone(), v.as_f64().unwrap())
                                                (k.clone(), v.clone())
                                            }
                                        )
                                        .collect();

    let bar = dict.get("bar").unwrap().as_object().unwrap();
    let symbol = dict.get("symbol").unwrap().as_str().unwrap();
    let _time = bar.get("t").unwrap().as_str().unwrap();
    let _open = bar.get("o").unwrap().as_f64().unwrap();
    let _high = bar.get("h").unwrap().as_f64().unwrap();
    let _low = bar.get("l").unwrap().as_f64().unwrap();
    let _close = bar.get("c").unwrap().as_f64().unwrap();
    let _volume = bar.get("v").unwrap().as_f64().unwrap();
    let _trades = bar.get("n").unwrap().as_f64().unwrap();
    // let _vwap = bar.get("vwap").unwrap().as_f64().unwrap();
    let message = format!(
    "{} at {} 
    Open: {}
    High: {}
    Low: {}
    Close: {}
    Volume: {}
    Number of Trades: {}", 
    symbol, _time, _open, _high, _low, _close, _volume, _trades);

    msg.channel_id.say(&ctx.http, message).await?;
    Ok(())
}
use std::env;
use std::error::Error;

use reqwest::{Client, Url};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::repls::CommandReplExt;
use teloxide::types::InputFile;

#[derive(Debug, Deserialize)]
struct CatApiResponse {
    id: String,
    url: String,
    width: u32,
    height: u32,
}

async fn get_cat_image_url() -> Result<Url, Box<dyn Error>> {
    let client = Client::new();

    let api_key = env::var("CAT_API_KEY")?;

    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(&api_key)?);

    let response = client
        .get("https://api.thecatapi.com/v1/images/search")
        .headers(headers)
        .send()
        .await?
        .json::<Vec<CatApiResponse>>()
        .await?;
// Assuming response.first() returns a Result type
    let url_string_result = match response.first() {
        None => Err(Box::new("cannot get response")),
        Some(response) => Ok(response.url.clone()),
    };

    let parsed_url_result = match url_string_result {
        Ok(url_string) => {
            // Use the Url::parse method and handle its Result
            match Url::parse(&url_string) {
                Ok(url) => Ok(url),
                Err(e) => Err(Box::new(format!("cannot parse url string: {}", e))),
            }
        }
        Err(e) => Err(Box::new(format!("{}", e)))
    };

    // Now you have the final Result<Url, Box<dyn std::error::Error>>
    match parsed_url_result {
        Ok(parsed_url) => {
            // Successfully parsed URL
            Ok(parsed_url)
        }
        Err(e) => Err(Box::from(format!("{}", &e)))
    }
    // match response.first() {
    //     None => { Err(Box::from("No response")) }
    //     Some(response) => {
    //         println!("serving image {:?}", response.url);
    //         match response.url.clone().parse::<Url>() {
    //             Err(_) => { Err(Box::from(format!("Cannot parse url {}", response.url))) }
    //             Ok(url) => { Ok(url) }
    //         }
    //     }
    // }.expect("TODO: panic message");
    // Ok(response.first().unwrap().url.clone().parse().unwrap())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Cat => {
            let image = get_cat_image_url().await;
            bot.send_photo(msg.chat.id, InputFile::url(image.unwrap())).await?;
        }
    };
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    Cat,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}
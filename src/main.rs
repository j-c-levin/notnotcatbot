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

    println!("serving image {:?}", response.first().unwrap().url);

    Ok(response.first().unwrap().url.clone().parse().unwrap())
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
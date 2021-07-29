use std::error::Error;
use teloxide::prelude::*;
use thirtyfour::WebDriver;

mod browser;
mod commands;
mod config;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
    let config = Config::from_env().unwrap();

    let driver: WebDriver = browser::init().await;
    match browser::login(&driver, &config).await {
        Ok(_) => {}
        Err(e) => {
            panic!("You can't connect: `{}`, credentials are {:?}", e, config);
        }
    };
    teloxide::commands_repl(bot, "unict-reservation", commands::handler).await;

    log::info!("Closing bot... Goodbye!");
    Ok(())
}

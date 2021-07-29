use std::error::Error;
use teloxide::prelude::*;
use thirtyfour::WebDriver;

mod browser;
mod commands;
mod config;

use crate::config::Config;

use tokio_stream::wrappers::UnboundedReceiverStream;

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

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                commands::handler(cx).await.log_on_error().await;
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                let data = &cx.update.data;
                if let Some(d) = data {
                    println!("{}", d);
                }
            })
        })
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");
    driver.quit().await?;

    Ok(())
}

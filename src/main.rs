use std::error::Error;
use teloxide::prelude::*;
mod browser;
mod callbacks;
mod commands;
mod config;
mod keyboard;

use crate::config::Config;

use tokio_stream::wrappers::UnboundedReceiverStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
    let config = Config::from_env().unwrap();

    unsafe {
        // Open the browser
        browser::init(&config).await;
    }

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                commands::handler(cx).await.log_on_error().await;
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                let data = &cx.update.data;
                if let Some(text) = data {
                    let _ = callbacks::handler(&cx, text).await;
                }
            })
        })
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}

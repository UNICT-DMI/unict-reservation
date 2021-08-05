use crate::browser;
use crate::config::Config;
use crate::keyboard::make_inline_keyboard;
use std::error::Error;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{AutoSend, Bot, Message, UpdateWithCx};
use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Reserve a classroom for tomorrow", parse_with = "split")]
    Room,
}

/// This is the handler for the commands.
/// It's called by `teloxide` every time someone write `/<text>`.
///
/// First it checks if the author is authorized to do the commands. Then match that text thanks to
/// the `parse` method of `BotCommand`.
pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // In the config file there's a key for the Telegram username authorized to use this bot.
    let username = Config::from_env().unwrap().username;

    // Compare the author of the message with the Telegram username from the config
    if let Some(author) = &cx.update.from().unwrap().username {
        if *author != username {
            cx.reply_to("You are not allowed to do this action!")
                .await?;

            return Ok(());
        }
    }

    let txt = cx.update.text();
    if txt.is_none() {
        return Ok(());
    }

    if let Ok(command) = BotCommand::parse(txt.unwrap(), "unict-reservation") {
        match command {
            Command::Help => {
                // Just send the descriptions of all commands
                cx.answer(Command::descriptions()).await?;
            }
            Command::Room => {
                let faculties;

                unsafe {
                    faculties = browser::get_faculties().await.unwrap();
                }

                let keyboard = make_inline_keyboard(&faculties, "faculty").await;
                cx.answer("Where?").reply_markup(keyboard).await?;
            }
        }
    } else {
        cx.reply_to("Command not found!").await?;
    }

    Ok(())
}

use crate::browser;
use crate::config::Config;
use std::error::Error;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{AutoSend, Bot, Message, UpdateWithCx};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
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

                // This is an array of array because the `InlineKeyboardMarkup`
                // considers each array as a row.
                // So, using this format Vec<Vec<..>> will print a button
                // in `n` different rows in only 1 column.
                let mut faculties_array: Vec<Vec<InlineKeyboardButton>> = vec![];
                unsafe {
                    faculties = browser::get_faculties().await.unwrap();
                }

                if let Some(faculties_texts) = faculties {
                    for (key, value) in faculties_texts {
                        faculties_array.push(vec![InlineKeyboardButton::callback(
                            value,
                            format!("faculty_{}", key),
                        )]);
                    }
                } else {
                    // If the response of the Option `faculties` is None, just answer
                    // an useless button with a text.
                    // I still don't know if it's a good idea to use a callback instead of
                    // a normal text button, but I could handle any such kind of callback
                    faculties_array.push(vec![InlineKeyboardButton::callback(
                        "No such element".to_string(),
                        "".into(),
                    )]);
                }

                // The `new` method accepts an interator
                let keyboard = InlineKeyboardMarkup::new(faculties_array);
                cx.answer("Where?").reply_markup(keyboard).await?;
            }
        }
    } else {
        cx.reply_to("Command not found!").await?;
    }

    Ok(())
}

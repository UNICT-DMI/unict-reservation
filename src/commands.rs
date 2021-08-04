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

pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let username = Config::from_env().unwrap().username;

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
                cx.answer(Command::descriptions()).await?;
            }
            Command::Room => {
                let faculties;
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
                    faculties_array.push(vec![InlineKeyboardButton::callback(
                        "No such element".to_string(),
                        "".into(),
                    )]);
                }

                let keyboard = InlineKeyboardMarkup::new(faculties_array);
                cx.answer("Where?").reply_markup(keyboard).await?;
            }
        }
    } else {
        cx.reply_to("Command not found!").await?;
    }

    Ok(())
}

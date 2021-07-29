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
                let url_button =
                    InlineKeyboardButton::callback("hello".to_string(), "hello_call".to_string());
                let keyboard = InlineKeyboardMarkup::default().append_row(vec![url_button]);
                cx.answer("Where?").reply_markup(keyboard).await?;
            }
        }
    } else {
        cx.reply_to("Command not found!").await?;
    }

    Ok(())
}

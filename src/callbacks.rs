use crate::browser;
use crate::keyboard::make_inline_keyboard;
use std::error::Error;
use teloxide::{
    payloads::EditMessageReplyMarkupSetters,
    prelude::{AutoSend, Bot, CallbackQuery, Requester, UpdateWithCx},
};

/// Handle the string of callback data
pub async fn handler(
    cx: &UpdateWithCx<AutoSend<Bot>, CallbackQuery>,
    text: &String,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let call: Vec<&str> = text.split("_").collect();

    let chat_id = call[0];
    // First part of `_` string is the type of callback we want to handle
    let type_ = call[1];

    match type_ {
        "faculty" => unsafe {
            // Select the faculty
            match browser::select_option("select2-results__option", "data-select2-id", call[2])
                .await
            {
                Ok(result) => {
                    if result {
                        let spaces = browser::get_spaces().await.unwrap();
                        let keyboard =
                            make_inline_keyboard(&spaces, "space", chat_id.parse::<i64>().unwrap())
                                .await;

                        // Edit the previous faculties message with spaces' buttons
                        cx.requester
                            .edit_message_reply_markup(
                                chat_id.to_string(),
                                cx.update.message.clone().unwrap().id,
                            )
                            .reply_markup(keyboard)
                            .await?;

                        return Ok(true);
                    } else {
                        return Ok(false);
                    }
                }
                Err(_) => {
                    return Ok(false);
                }
            }
        },
        "space" => unsafe {
            // Select the sapce
            match browser::select_option("select2-results__option", "data-select2-id", call[2])
                .await
            {
                Ok(result) => {
                    if result {
                        let timetable = browser::get_timetable().await.unwrap();
                        let keyboard = make_inline_keyboard(
                            &timetable,
                            "timetable",
                            chat_id.parse::<i64>().unwrap(),
                        )
                        .await;

                        // Edit the previous spaces message with timetable' buttons
                        cx.requester
                            .edit_message_text(
                                chat_id.to_string(),
                                cx.update.message.clone().unwrap().id,
                                "When?",
                            )
                            .await?;

                        cx.requester
                            .edit_message_reply_markup(
                                chat_id.to_string(),
                                cx.update.message.clone().unwrap().id,
                            )
                            .reply_markup(keyboard)
                            .await?;

                        return Ok(true);
                    } else {
                        return Ok(false);
                    }
                }
                Err(_) => {
                    return Ok(false);
                }
            }
        },
        _ => Ok(false),
    }
}

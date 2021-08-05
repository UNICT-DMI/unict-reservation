use std::collections::HashMap;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

/// Create a new InlineKeyboardMarkup from an `hashmap` defined as `<id>:<text>`
pub async fn make_inline_keyboard(
    hashmap: &Option<HashMap<String, String>>,
    callback_type: &str,
) -> InlineKeyboardMarkup {
    // This is an array of array because the `InlineKeyboardMarkup`
    // considers each array as a row.
    // So, using this format Vec<Vec<..>> will print a button
    // in `n` different rows in only 1 column.
    let mut keyboard_array: Vec<Vec<InlineKeyboardButton>> = vec![];

    if let Some(options) = hashmap {
        for (key, value) in options {
            keyboard_array.push(vec![InlineKeyboardButton::callback(
                value.clone(),
                format!("{}_{}", callback_type, key),
            )]);
        }
    } else {
        // If the response of the Option ``callback_type`` is None, just answer
        // an useless button with a text.
        // I still don't know if it's a good idea to use a callback instead of
        // a normal text button, but I could handle any such kind of callback
        keyboard_array.push(vec![InlineKeyboardButton::callback(
            "No such element".to_string(),
            "".into(),
        )]);
    }

    // The `new` method accepts an interator
    return InlineKeyboardMarkup::new(keyboard_array);
}

use crate::browser;

/// Handle the string of callback data
pub async fn handler(text: &String) {
    let call: Vec<&str> = text.split("_").collect();

    // First part of `_` string is the type of callback we want to handle
    let type_ = call[0];

    match type_ {
        "faculty" => unsafe {
            // Select the faculty
            let _ =
                browser::select_option("select2-results__option", "data-select2-id", call[1]).await;
        },
        _ => {}
    };
}

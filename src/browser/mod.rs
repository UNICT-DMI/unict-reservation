use std::collections::HashMap;

use self::web_browser::{Browser, WEB_BROWSER};
use crate::Config;
use thirtyfour::prelude::WebDriverResult;

mod web_browser;

/// Create a new instance of `Browser` and associate it with the static variable `WEB_BROWSER`.
/// This is an unsecure type of usage, so the block is inside the `unsafe` block
pub async fn init(driver_url: &String) {
    unsafe {
        WEB_BROWSER = Some(Browser::new(driver_url).await);
    }
}

/// Login using the credentials from the `Config`. 'Cause its kind of nature
/// this is an `unsafe` block, so the function is defined like that
pub async unsafe fn login(credentials: &Config) -> WebDriverResult<()> {
    if let Some(driver) = &WEB_BROWSER {
        driver._login(credentials).await?;
    }

    Ok(())
}

/// Get the faculties available for booking a room
pub async unsafe fn get_faculties() -> WebDriverResult<Option<HashMap<String, String>>> {
    if let Some(driver) = &WEB_BROWSER {
        if let Some(faculties) = driver.faculties().await? {
            return Ok(Some(faculties));
        }
    }

    Ok(None)
}

/// Get the spaces (rooms) available to book
pub async unsafe fn get_spaces() -> WebDriverResult<Option<HashMap<String, String>>> {
    if let Some(driver) = &WEB_BROWSER {
        if let Some(spaces) = driver.spaces().await? {
            return Ok(Some(spaces));
        }
    }

    Ok(None)
}

pub async unsafe fn select_option(
    klass: &str,
    property_name: &str,
    property_value: &str,
) -> WebDriverResult<bool> {
    if let Some(driver) = &WEB_BROWSER {
        let result = driver
            .select_option_from_list(klass, property_name, property_value)
            .await?;

        return Ok(result);
    }

    Ok(false)
}

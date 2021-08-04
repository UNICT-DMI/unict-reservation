use std::collections::HashMap;

use self::web_browser::{Browser, WEB_BROWSER};
use crate::Config;
use thirtyfour::prelude::WebDriverResult;

mod web_browser;

pub async fn init(driver_url: &String) {
    unsafe {
        WEB_BROWSER = Some(Browser::new(driver_url).await);
    }
}

pub async unsafe fn login(credentials: &Config) -> WebDriverResult<()> {
    if let Some(driver) = &WEB_BROWSER {
        driver._login(credentials).await?;
    }

    Ok(())
}

pub async unsafe fn get_faculties() -> WebDriverResult<Option<HashMap<String, String>>> {
    if let Some(driver) = &WEB_BROWSER {
        if let Some(faculties) = driver.faculties().await? {
            return Ok(Some(faculties));
        }
    }

    Ok(None)
}

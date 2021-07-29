use crate::config::Config;
use std::{thread, time};
use thirtyfour::prelude::{By, WebDriverResult};
use thirtyfour::{FirefoxCapabilities, WebDriver, WebDriverCommands};

pub async fn init() -> WebDriver {
    let driver = match WebDriver::new("http://localhost:4444", FirefoxCapabilities::new()).await {
        Ok(driver) => driver,
        Err(e) => {
            panic!(e);
        }
    };

    return driver;
}

pub async fn login(driver: &WebDriver, credentials: &Config) -> WebDriverResult<()> {
    driver
        .get("https://studenti.smartedu.unict.it/WorkFlow2011/Logon/Logon.aspx")
        .await?;

    let cf_input = driver
        .find_element(By::Name("ctl01$contents$UserName"))
        .await?;
    cf_input.send_keys(&credentials.cf).await?;

    let psw_input = driver
        .find_element(By::Name("ctl01$contents$UserPassword"))
        .await?;
    psw_input.send_keys(&credentials.password).await?;

    thread::sleep(time::Duration::from_millis(1000));

    driver
        .find_element(By::Name("ctl01$contents$LogonButton"))
        .await?
        .click()
        .await?;

    Ok(())
}

use crate::Config;
use std::{thread, time};
use thirtyfour::error::{WebDriverError, WebDriverErrorInfo, WebDriverErrorValue};
use thirtyfour::prelude::{By, WebDriverResult};
use thirtyfour::{FirefoxCapabilities, WebDriver, WebDriverCommands};

const LOGIN_URL: &str = "https://studenti.smartedu.unict.it/WorkFlow2011/Logon/Logon.aspx";

pub struct Browser {
    driver: Option<WebDriver>,
}

impl Browser {
    pub async fn new(driver_url: &String) -> Self {
        Self {
            driver: Some(
                WebDriver::new(driver_url, FirefoxCapabilities::new())
                    .await
                    .unwrap(),
            ),
        }
    }

    // TODO: Close the driver fixing the borrowing error
    // async fn _close(self) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }

    pub async unsafe fn _login(&self, credentials: &Config) -> WebDriverResult<()> {
        if let Some(_d) = &self.driver {
            _d.get(LOGIN_URL).await?;

            let cf_input = _d.find_element(By::Name("ctl01$contents$UserName")).await?;
            cf_input.send_keys(&credentials.cf).await?;

            let psw_input = _d
                .find_element(By::Name("ctl01$contents$UserPassword"))
                .await?;
            psw_input.send_keys(&credentials.password).await?;

            thread::sleep(time::Duration::from_millis(1000));

            _d.find_element(By::Name("ctl01$contents$LogonButton"))
                .await?
                .click()
                .await?;

            thread::sleep(time::Duration::from_millis(2000));

            if _d.current_url().await? == LOGIN_URL {
                return Err(WebDriverError::SessionNotCreated(WebDriverErrorInfo {
                    status: 400,
                    error: "SessionNotCreated".to_string(),
                    value: WebDriverErrorValue {
                        message: "SessionNotCreated".to_string(),
                        error: None,
                        stacktrace: None,
                        data: None,
                    },
                }));
            }
        }

        Ok(())
    }
}

pub static mut WEB_BROWSER: Option<Browser> = None;

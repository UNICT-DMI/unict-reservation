use crate::Config;
use std::collections::HashMap;
use std::{thread, time};
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::error::{WebDriverError, WebDriverErrorInfo, WebDriverErrorValue};
use thirtyfour::prelude::{By, WebDriverResult};
use thirtyfour::{FirefoxCapabilities, WebDriver, WebDriverCommands};

/// This url is used to make the login
const LOGIN_URL: &str = "https://studenti.smartedu.unict.it/WorkFlow2011/Logon/Logon.aspx?ReturnUrl=%2fStudenti%2fDefault.aspx";
/// This url is used to go to the page where a student can book a room for study
const ROOMS_URL: &str = "https://studenti.smartedu.unict.it/StudentSpaceReserv?Type=unaTantum";

/// Browser struct
pub struct Browser {
    /// The driver for Firefox, it could be `None`
    driver: Option<WebDriver>,
}

impl Browser {
    /// Create a new `Browser` with a Firefox driver with a personalized User-Agent
    pub async fn new(driver_url: &String) -> Self {
        let user_agent =
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0";

        let mut prefs = FirefoxPreferences::new();
        let _ = prefs.set_user_agent(user_agent.to_string());

        let mut caps = FirefoxCapabilities::new();
        let _ = caps.set_preferences(prefs);

        Self {
            driver: Some(WebDriver::new(driver_url, caps).await.unwrap()),
        }
    }

    // TODO: Close the driver fixing the borrowing error
    // async fn _close(self) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }

    /// Login on `LOGIN_URL`
    pub async unsafe fn _login(&self, credentials: &Config) -> WebDriverResult<()> {
        if let Some(_d) = &self.driver {
            _d.get(LOGIN_URL).await?;

            let cf_input = _d.find_element(By::Name("ctl01$contents$UserName")).await?;
            cf_input.send_keys(&credentials.cf).await?;

            thread::sleep(time::Duration::from_millis(3000));

            let psw_input = _d
                .find_element(By::Name("ctl01$contents$UserPassword"))
                .await?;
            psw_input.send_keys(&credentials.password).await?;

            thread::sleep(time::Duration::from_millis(3000));

            _d.find_element(By::Name("ctl01$contents$LogonButton"))
                .await?
                .click()
                .await?;

            thread::sleep(time::Duration::from_millis(2000));

            // If the current url is the same as `LOGIN_URL` it means the login didn't work, so
            // returns a "login error"
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

    /// Get all faculties for booking and return an `HashMap<key, value>` when `key` is the
    /// key for that faculty inside the `select` tag and `value` is just the text of the option.
    pub async fn faculties(&self) -> WebDriverResult<Option<HashMap<String, String>>> {
        if let Some(_d) = &self.driver {
            _d.get(ROOMS_URL).await?;
            thread::sleep(time::Duration::from_millis(1000));

            _d.find_element(By::Css(
                "span[aria-labelledby='select2-dipartimentoSelector-container']",
            ))
            .await?
            .click()
            .await?;

            let list_elements = _d
                .find_elements(By::Css("#select2-dipartimentoSelector-results li"))
                .await?;

            let mut faculties_ids = HashMap::<String, String>::new();

            for i in list_elements {
                faculties_ids.insert(
                    i.get_attribute("data-select2-id").await.unwrap().unwrap(),
                    i.text().await.unwrap(),
                );
            }

            return Ok(Some(faculties_ids));
        }

        Ok(None)
    }

    /// Get all spaces for booking and return an `HashMap<key, value>` when `key` is the
    /// key for that space inside the `select` tag and `value` is just the text of the option.
    pub async fn spaces(&self) -> WebDriverResult<Option<HashMap<String, String>>> {
        if let Some(_d) = &self.driver {
            thread::sleep(time::Duration::from_millis(1000));

            _d.find_element(By::Css(
                "span[aria-labelledby='select2-spaceSelector-container']",
            ))
            .await?
            .click()
            .await?;

            let list_elements = _d
                .find_elements(By::Css("#select2-spaceSelector-results li"))
                .await?;

            let mut spaces_ids = HashMap::<String, String>::new();

            for i in list_elements {
                spaces_ids.insert(
                    i.get_attribute("data-select2-id").await.unwrap().unwrap(),
                    i.text().await.unwrap(),
                );
            }

            return Ok(Some(spaces_ids));
        }

        Ok(None)
    }

    /// Select an option from a list of select elements
    pub async fn select_option_from_list(
        &self,
        klass: &str,
        property_name: &str,
        property_value: &str,
    ) -> WebDriverResult<bool> {
        if let Some(_d) = &self.driver {
            _d.find_element(By::Css(
                &format!("li.{}[{}='{}']", klass, property_name, property_value).to_owned()[..],
            ))
            .await?
            .click()
            .await?;

            return Ok(true);
        }

        Ok(false)
    }
}

/// The static unsafe variable used to open a web browser
pub static mut WEB_BROWSER: Option<Browser> = None;

use crate::Config;
use std::collections::HashMap;
use std::{thread, time};
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::error::{WebDriverError, WebDriverErrorInfo, WebDriverErrorValue};
use thirtyfour::prelude::{By, WebDriverResult};
use thirtyfour::{FirefoxCapabilities, WebDriver, WebDriverCommands, WebElement};

/// This url is used to make the login
const LOGIN_URL: &str = "https://studenti.smartedu.unict.it/WorkFlow2011/Logon/Logon.aspx";
/// This url is used to go to the page where a student can book a room for study
pub const ROOMS_URL: &str = "https://studenti.smartedu.unict.it/StudentSpaceReserv?Type=unaTantum";

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
        let _ = caps.set_headless();

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
            if _d.current_url().await?.starts_with(LOGIN_URL) {
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

    /// Get all options for booking following the `label` and return an `HashMap<key, value>` when `key` is the
    /// key for that label inside the `select` tag and `value` is just the text of the option.
    /// If `url` is specified then go to that page
    pub async fn get_options(
        &self,
        label: &str,
        url: &str,
    ) -> WebDriverResult<Option<HashMap<String, String>>> {
        if let Some(_d) = &self.driver {
            if url != "" {
                _d.get(url).await?;
            }
            thread::sleep(time::Duration::from_millis(1000));

            _d.find_element(By::Css(
                &format!(
                    "span[aria-labelledby='select2-{}Selector-container']",
                    label
                )
                .to_owned()[..],
            ))
            .await?
            .click()
            .await?;

            let list_elements = _d
                .find_elements(By::Css(
                    &format!("#select2-{}Selector-results li", label).to_owned()[..],
                ))
                .await?;

            let mut options = HashMap::<String, String>::new();

            for i in list_elements {
                options.insert(
                    i.get_attribute("data-select2-id").await.unwrap().unwrap(),
                    i.text().await.unwrap(),
                );
            }

            return Ok(Some(options));
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

    /// Get the timetable of available hours from the driver and returns an hashmap with id->text
    pub async fn get_timetable(&self) -> WebDriverResult<Option<HashMap<String, String>>> {
        if let Some(_d) = &self.driver {
            thread::sleep(time::Duration::from_millis(2000));

            let table_trs = _d
                .find_elements(By::Css("div[data-select2-id='studyPlan'] table tbody tr"))
                .await?;

            let mut timetable = HashMap::<String, String>::new();

            for i in table_trs.iter() {
                let cols: Vec<WebElement> = i.find_elements(By::Css("td")).await?;
                if cols.len() < 6 {
                    continue;
                }

                let col_id = i.find_element(By::Css("th")).await?.text().await.unwrap();
                let mut text_formatted = cols[0].text().await.unwrap();
                text_formatted.push_str(
                    &format!(
                        ", {} - {}.\n",
                        cols[1].text().await.unwrap(),
                        cols[2].text().await.unwrap()
                    )
                    .to_owned()[..],
                );

                text_formatted
                    .push_str(&format!("Posti: {}", cols[6].text().await.unwrap()).to_owned()[..]);

                timetable.insert(col_id, text_formatted);
            }

            return Ok(Some(timetable));
        }

        Ok(None)
    }

    // Select the row for the timetable booking
    pub async unsafe fn select_timetable_row(&self, index: &str) -> WebDriverResult<bool> {
        if let Some(_d) = &self.driver {
            _d.find_element(By::Css(
                &format!("#slotContainerTable tr:nth-child({}) td", index).to_owned()[..],
            ))
            .await?
            .click()
            .await?;
            thread::sleep(time::Duration::from_millis(2000));
            _d.find_element(By::Css("#partialQuestionYesNoConfirmButton:last-child"))
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

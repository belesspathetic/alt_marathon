use thirtyfour::prelude::*;

use crate::{
    browser::{close, start},
    utils::async_wait_n_sec,
};

pub struct VideoInfo {
    pub title: String,
    pub href: String,
}

impl VideoInfo {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let browser = start().await?;
        let driver = &browser.driver;

        let el = Self::get_element(&driver, url).await?;
        let title = Self::get_title(&el).await?;
        let href = Self::get_href(&el).await?;

        close(browser).await?;

        Ok(Self {
            title: title,
            href: href,
        })
    }

    async fn get_element(driver: &WebDriver, url: &str) -> anyhow::Result<WebElement> {
        driver.goto(url).await?;
        async_wait_n_sec(3).await?;
        let el = driver.find(By::Id("video-title-link")).await?;

        Ok(el)
    }

    async fn get_title(el: &WebElement) -> anyhow::Result<String> {
        let href = el.attr("title").await?.unwrap();

        Ok(href)
    }

    async fn get_href(el: &WebElement) -> anyhow::Result<String> {
        let href = el.attr("href").await?.unwrap();

        Ok(href)
    }
}

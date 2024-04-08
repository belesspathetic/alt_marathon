use alt_marathon::browser::{close, start};
use alt_marathon::utils::async_wait_n_sec;
use thirtyfour::prelude::*;

const TEST_URL: &str = "https://www.youtube.com/@STERNENKO/videos";

#[tokio::test]
async fn get_video_id() -> anyhow::Result<()> {
    let browser = start().await?;
    let driver = &browser.driver;

    driver.goto("https://youtube.com").await?;
    driver.goto(TEST_URL).await?;
    async_wait_n_sec(3).await?;

    let t = driver.find(By::Id("video-title-link")).await?;

    let attr = t.attr("href").await?.unwrap();

    let title = t.attr("title").await?.unwrap();

    println!("TITLE:{}\nLINK:{}", title, attr);

    assert!(!attr.is_empty());
    assert!(!title.is_empty());

    close(browser).await?;
    Ok(())
}

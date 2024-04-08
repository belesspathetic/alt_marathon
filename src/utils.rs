use std::env;

const ACCESS_TOKEN: &str = "ACCESS_TOKEN";
const PAGE_ID: &str = "PAGE_ID";

pub fn read_env() -> (String, String) {
    if !is_env_key_set() {
        panic!("FAIL: Please set api key and page id in env variable");
    } else {
        return (env::var(ACCESS_TOKEN).unwrap(), env::var(PAGE_ID).unwrap());
    }
}

fn is_env_key_set() -> bool {
    let mbool = match env::var(ACCESS_TOKEN) {
        Ok(_) => true,
        Err(_) => false,
    };

    mbool
}

pub async fn async_wait_n_sec(n: u64) -> anyhow::Result<()> {
    let delay = tokio::time::Duration::from_secs(n);
    tokio::time::sleep(delay).await;

    Ok(())
}

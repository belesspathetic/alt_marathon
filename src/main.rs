use std::env;

use alt_marathon::{
    mode::{auto, manual, manual_prompt},
    utils::read_env,
};
use fb_poster::utils::Secrets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set API key
    let (access_token, page_id) = read_env();
    let secrets = Secrets::new(access_token.as_str(), page_id.as_str());
    // Get flags
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--manual".to_string()) {
        let input = manual_prompt();
        manual(secrets, input).await?;
    } else {
        auto(secrets).await?;
    }

    Ok(())
}

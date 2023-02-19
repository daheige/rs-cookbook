use std::time::Duration;

use anyhow::Result;
use reqwest::{self, ClientBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    let user_agent =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/109.0";

    println!("request url:{}", request_url);
    // 创建请求的client
    let timeout = Duration::from_secs(5);
    let client = ClientBuilder::new().timeout(timeout).build()?;

    // 发送异步请求，然后调用await等待结果
    let resp = client
        .get(&request_url)
        .header("User-Agent", user_agent)
        .send()
        .await?;

    // println!("body: {:?}", resp.text().await?);
    let users: Vec<User> = resp.json().await?;
    println!("users: {:?}", users);

    Ok(())
}

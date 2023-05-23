use anyhow::Result;
use reqwest::{self, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    id: i64,
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

// 通过reqwest 实现文件下载功能
#[tokio::test]
async fn test_download() -> Result<()> {
    use std::{
        fs::{self, File},
        io::copy,
        path::Path,
    };

    let tmp_dir = Path::new("../download");
    fs::create_dir_all(tmp_dir)?;
    println!("tmp_dir: {:?}", tmp_dir);
    let target_url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let resp = reqwest::get(target_url).await?;

    // 指定下载后的文件名
    let mut dest = {
        let fname = resp
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap();
        println!("file to download :{}", fname);
        let fname = tmp_dir.join(fname);
        File::create(fname)?
    };

    let content = resp.text().await?;
    // 将下载的文件内容复制到指定的目标文件
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
/*
running 1 test
tmp_dir: "../log"
file to download :rust-logo-512x512.png
test test_download ... ok
 */

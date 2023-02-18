use anyhow::Result;
use reqwest::{self, ClientBuilder};
use std::{io::Read, time::Duration};

fn main() -> Result<()> {
    println!("client request demo...");
    // 以block client的方式请求，也就是同步执行请求
    let mut response = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new(); // 存放响应的body
    response.read_to_string(&mut body)?;
    println!("http status: {}", response.status());
    println!("http headers: {:#?}", response.headers());
    // let content_type = response.headers().get("content-type").unwrap();
    // println!("resp content-type: {}", content_type.to_str().unwrap());
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .unwrap();
    println!("resp content-type: {}", content_type.to_str().unwrap());

    println!("resp body: {}", body);
    Ok(())
}

// tokio 测试写法
#[tokio::test]
async fn test_async_request() -> Result<()> {
    // 异步的方式请求 async/await组合
    let response = reqwest::get("http://httpbin.org/get").await?;

    println!("http status: {}", response.status());
    println!("http headers: {:#?}", response.headers());
    let body = response.text().await?;
    println!("resp body: {}", body);
    Ok(())
}

#[tokio::test]
async fn check_user_exist() -> Result<()> {
    let user = "daheige";
    let request_url = format!("https://api.github.com/users/{}", user);
    let timeout = Duration::new(5, 0);

    let user_agent =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/109.0";
    // 创建请求的client
    let client = ClientBuilder::new().timeout(timeout).build()?;

    // 发送异步请求，然后调用await等待结果
    let response = client
        .get(&request_url)
        .header("User-Agent", user_agent)
        .send()
        .await?;
    if response.status().is_success() {
        let body = response.text().await?;
        println!("resp body: {}", body);
    } else {
        println!("http status: {}", response.status());
        println!("{} is not a user", user);
    }

    Ok(())
}

use mime;
use reqwest::header::CONTENT_TYPE;

use std::{collections::HashMap, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // mime类型解析
    let plain_text: mime::Mime = "text/plain".parse().unwrap();
    assert_eq!(plain_text, mime::TEXT_PLAIN);
    println!("test ok");

    // 从字符串获取 MIME 类型
    let json_type = "application/json";
    let parsed_mime = json_type
        .parse::<mime::Mime>()
        .unwrap_or(mime::APPLICATION_OCTET_STREAM);
    print!("MIME for {:?} was parsed as {:?}", json_type, parsed_mime);

    // 从文件名获取 MIME 类型
    let filenames = vec!["abc.jpg", "foo.json", "foobar.png", "text.md"];
    for file in filenames {
        let mime_type = find_mime_type(&file.to_owned());
        println!("current {} type is {}", file, mime_type);
    }

    // reqwest api请求
    let resp = reqwest::get("https://httpbin.org/ip").await?;
    let headers = resp.headers();

    // response type判断
    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
            let content_type = mime::Mime::from_str(content_type.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::APPLICATION, mime::JSON) => "json type",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };
            println!("The reponse contains {}.", media_type);
        }
    };

    // 打印header头
    println!("headers:{:?}", headers);
    let result = resp.json::<HashMap<String, String>>().await?;
    println!("rsp: {:#?}", result);

    Ok(())
}

// 根据文件名获取mime type
fn find_mime_type(filename: &String) -> mime::Mime {
    let parts: Vec<&str> = filename.split(".").collect();
    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            _ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };

    res
}

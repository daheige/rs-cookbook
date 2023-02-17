use mime;
fn main() {
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

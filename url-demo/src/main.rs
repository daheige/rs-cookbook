use url::form_urlencoded::{byte_serialize, parse};
use url::{Origin, ParseError, Position, Url};

fn main() {
    /*
    使用 form_urlencoded::byte_serialize 将字符串编码为 application/x-www-form-urlencoded
    表单语法，随后使用 form_urlencoded::parse 对其进行解码。
    这两个函数都返回迭代器，然后这些迭代器聚集为 String
     */
    let url_str = "https://github.com/daheige?u=1";
    // url encode
    let u: String = byte_serialize(url_str.as_bytes()).collect();
    println!("u:{}", u);

    // url parse
    let s: String = parse(u.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    println!("decode str:{}", s);

    assert_eq!(s, url_str);

    let user_url = build_url("https://github.com", "/daheige").unwrap();
    println!("user_url:{}", user_url);
}

// 从基本 URL 创建新 URLs
// url路径拼接
fn build_url(base_url: &str, path: &str) -> Result<Url, ParseError> {
    let base = Url::parse(base_url).expect("base_url invalid");
    let joined = base.join(path)?;
    Ok(joined)
}

#[test]
fn url_parse() -> Result<(), ParseError> {
    // 提取url各个部分
    let s = "https://github.com/daheige?i=1&b=2&c=rust";
    let parsed = Url::parse(s)?;
    println!("url path: {}", parsed.path());
    println!("domain:{:?}", parsed.domain());
    println!("host:{:?}", parsed.host());
    println!("port:{:?}", parsed.port().unwrap_or(443));
    println!("https port:{:?}", parsed.port_or_known_default());
    println!("scheme:{:?}", parsed.scheme());

    // 也可以通过origin获取相关的信息
    let origin = parsed.origin();
    match origin {
        Origin::Tuple(scheme, domain, port) => {
            println!("scheme from origin:{}", scheme);
            println!("domain from origin:{}", domain.to_string());
            println!("port from origin:{}", port);
        }
        _ => {
            return Err(ParseError::InvalidIpv4Address);
        }
    }

    println!("origin:{:?}", parsed.origin());

    // 从 URL 移除片段标识符和查询对
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned:{}", cleaned);

    let user_url = build_url("https://github.com", "/daheige")?;
    println!("user_url:{}", user_url);

    Ok(())
}

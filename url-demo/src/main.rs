use url::form_urlencoded::{byte_serialize, parse};
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
}

use base64::{engine::general_purpose::STANDARD, Engine as _};
use data_encoding::{DecodeError, HEXUPPER};
use std::str;

/*
data_encoding crate 提供了 HEXUPPER::encode 方法，
该方法接受 &[u8] 参数并返回十六进制数据的字符串 String。
类似地，data_encoding crate 提供了 HEXUPPER::decode 方法，
该方法接受 &[u8] 参数。如果输入数据被成功解码，则返回 Vec<u8>。
*/
fn main() -> Result<(), DecodeError> {
    let original = b"abcd hello rust";

    // 十六进制编码
    let s = HEXUPPER.encode(original);
    println!("{}", s);

    let expected = "616263642068656C6C6F2072757374";
    assert_eq!(s, expected);

    // 十六进制解码
    let d = HEXUPPER.decode(s.as_bytes())?;
    assert_eq!(&d[..], &original[..]);

    let hello = b"hello,rustaceans";

    // Base64 编码
    let encoded = STANDARD.encode(hello);
    println!("base64 encode str: {}", encoded);

    // Base64 解码
    let decoded = STANDARD.decode(&encoded).unwrap();

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 decode: {}", str::from_utf8(&decoded).unwrap());

    Ok(())
}

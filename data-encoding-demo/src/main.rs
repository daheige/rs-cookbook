use data_encoding::{DecodeError, HEXUPPER};

// base64
use base64::{encode,decode};
use std::str;
/*
data_encoding crate 提供了 HEXUPPER::encode 方法，
该方法接受 &[u8] 参数并返回十六进制数据的字符串 String。
类似地，data_encoding crate 提供了 HEXUPPER::decode 方法，
该方法接受 &[u8] 参数。如果输入数据被成功解码，则返回 Vec<u8>
 */
fn main()->Result<(),DecodeError> {
    let original = b"abcd hello rust";
    let s = HEXUPPER.encode(original);
    println!("{}",s);
    let expected = "616263642068656C6C6F2072757374";
    assert_eq!(s,expected);

    // decode
    let d = HEXUPPER.decode(&s.into_bytes())?;
    assert_eq!(&d[..],&original[..]);

    let hello = b"hello,rustaceans";
    // base encode
    let encoded = encode(hello);
    println!("base64 encode str:{}",encoded);

    // base64 decode
    let decoded = decode(&encoded).unwrap();
    println!("origin:{}",str::from_utf8(hello).unwrap());

    println!("base64 decode:{}",str::from_utf8(&decoded).unwrap());
    Ok(())
}
/*
output:
616263642068656C6C6F2072757374
base64 encode str:aGVsbG8scnVzdGFjZWFucw==
origin:hello,rustaceans
base64 decode:hello,rustaceans
 */
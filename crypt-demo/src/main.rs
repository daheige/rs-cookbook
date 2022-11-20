use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::{write, File};
use std::io::{BufReader, Error, Read, Write};

// hmac check
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{hmac, rand};

fn main() {
    println!("Hello, world!");
    write_and_read_sha256();
    check_hmac();
}

// 计算文件的hash值
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut ctx = Context::new(&SHA256);
    let mut buf = [0; 1024];
    loop {
        let count = reader.read(&mut buf)?;
        if count == 0 {
            break;
        }
        ctx.update(&buf[..count]);
    }

    Ok(ctx.finish())
}

// 先创建文件，写入一些数据。然后使用 digest::Context 计算文件内容的 SHA-256 摘要 digest::Digest
fn write_and_read_sha256() -> Result<(), std::io::Error> {
    let path = "test.md";
    let mut output = File::create(path)?;
    write!(output, "gen a digest of this file")?;
    let input = File::open(path)?;
    let reader = BufReader::new(input); // 这里直接返回reader
    let digest = sha256_digest(reader)?;

    println!("sha-256 of this file: {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}

// 使用 HMAC 摘要对消息进行签名和验证
fn check_hmac() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;

    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "hello hmac test";
    let signature = hmac::sign(&key, message.as_bytes());
    // println!("sign:{:?}", signature.as_ref());
    println!("sign:{:?}", signature);

    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    Ok(())
}

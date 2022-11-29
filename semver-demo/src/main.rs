use semver::{Error, Version};
fn main() -> Result<(), Error> {
    let mut parsed_version = Version::parse("0.2.6")?;
    assert_eq!(parsed_version, Version::new(0, 2, 6));

    parsed_version.patch += 1;
    assert_eq!(parsed_version.to_string(), "0.2.7");
    println!("new version:{}", parsed_version.to_string());

    parsed_version.minor += 1;
    println!("new version:{}", parsed_version.to_string());

    parsed_version.major += 1;
    println!("new version:{}", parsed_version.to_string());

    let v2 = Version::parse("0.2.7")?;
    let b = parsed_version.cmp(&v2).is_gt();
    println!("parsed_version < v2 :{}", b);

    Ok(())
}

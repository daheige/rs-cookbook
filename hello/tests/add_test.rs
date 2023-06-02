#[test]
fn add() {
    let x: i32 = 12;
    println!("hello,tests");
    println!("{}", x + 1);
    let y = 2 + 1;
    assert_eq!(y, 3);
}

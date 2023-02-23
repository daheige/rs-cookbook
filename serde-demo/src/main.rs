use serde::{Deserialize, Serialize};

// Attribute macro used to apply derive macros
// user能序列化和反序列化derive定义
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i64,
    name: String,
    age: i32,

    // 重命名字段
    #[serde(rename = "b_id")]
    b_id: String,

    // 忽略某个字段
    #[serde(skip)]
    flag: i32,
}

fn main() {
    println!("Hello, world!");
    let u = User {
        id: 1,
        name: "daheige".to_string(),
        age: 33,
        b_id: "test-user".to_string(),
        flag: 1,
    };

    let s = serde_json::to_string(&u).unwrap();
    println!("json: {}", s);

    let s = r#"{"id":1,"name":"daheige","age":33,"b_id":"test-user","flag":1}"#;
    let user: User = serde_json::from_str(s).unwrap();
    println!("user:{:?}", user);
    println!("flag:{}", user.flag);
}

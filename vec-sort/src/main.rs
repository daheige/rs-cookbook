// 为了使得结构体可以排序，需要实现4个trait: Eq,PartialEq,Ord,PartialOrd
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

// 为Person添加静态方法，实现 Person 实例创建
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

fn main() {
    let mut v = vec![1, 12, 9, 2, 3, 6, 4, 5];
    v.sort();
    println!("v sort asc: {:?}", v);

    // 浮点数排序 sort_by 方式排序
    let mut f = vec![1.1, 1.2, 1.21, 1.13, 2.1, 1.5];
    f.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("f sort asc: {:?}", f);

    // 对结构体排序
    let mut list = vec![
        Person::new("daheige".to_string(), 32),
        Person::new("zoe".to_string(), 21),
        Person::new("al".to_string(), 46),
    ];

    // 根据自然排序
    list.sort();
    println!("{:?}", list);

    // 根据age从大到小排序
    list.sort_by(|a, b| b.age.cmp(&a.age));
    println!("sort by age desc: {:?}", list);

    list.sort_by(|a, b| a.age.cmp(&b.age));
    println!("sort by age asc: {:?}", list);
}

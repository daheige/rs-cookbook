use std::collections::HashMap;

struct ColumnEntity {
    key: String,
    num: i32,
}

fn main() {
    // map是一个key/vec类型
    let mut m = HashMap::new();
    let nums = vec![1, 2, 3];
    m.insert("a", nums);
    let nums2 = vec![1, 2, 3];
    m.insert("b", nums2);

    let records: Vec<ColumnEntity> = vec![
        ColumnEntity {
            key: "a".to_string(),
            num: 5,
        },
        ColumnEntity {
            key: "a".to_string(),
            num: 6,
        },
        ColumnEntity {
            key: "b".to_string(),
            num: 8,
        },
    ];

    // for record in &records {
    //     if m.contains_key(record.key.as_str()) {
    //         // 如果m包含key，就获得m元素的可变引用
    //         let nums = m.get_mut(record.key.as_str()).unwrap();
    //         nums.push(record.num);
    //     } else {
    //         // 不存在key，就插入
    //         m.insert(record.key.as_str(), vec![record.num]);
    //     }
    // }

    // 上面的例子简写模式
    for record in &records {
        // entry方法是获取map的元素，返回map元素的可变引用
        // 接着调用or_insert表示不存在就插入一个元素
        // 当这两个方法执行完毕后，返回一个可变引用类型 &mut Vec<i32>
        let nums = m.entry(&record.key).or_insert(vec![]);
        nums.push(record.num);
    }

    println!("{:?}", m);

    // 统计单词出现的次数
    let s = "hello abc world eee abc rust world";
    let mut m = HashMap::new(); // m是一个可变引用hash map
    for word in s.split_whitespace() {
        // 按照空格进行分割为一个切片类型
        let num = m.entry(word).or_insert(0);
        *num += 1;
    }

    println!("m:{:?}", m);
}

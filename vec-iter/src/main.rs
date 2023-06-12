fn main() {
    let mut nums: Vec<i64> = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);

    // 对集合元素进行遍历
    // 1.通过借用的方式遍历
    // 此时val是一个不可变引用类型
    for val in &nums {
        println!("current num:{}", val);
    }

    // 2.通过索引方式访问key,val eg: key:0 value:1
    // iter()引用的方式迭代，将其变成一个迭代，然后通过enumerate()获取（key,&val)元组类型
    for (k, val) in nums.iter().enumerate() {
        println!("key:{} value:{}", k, val);
    }

    // iter_mut() 可变引用方式迭代
    // val 是一个&mut i64可变引用类型
    for val in nums.iter_mut() {
        // 通过*val解引用的方式获得nums元素
        *val += 1;
    }

    println!("nums:{:?}", nums); // nums:[2, 3, 4]

    // 3. 直接通过for in遍历
    // 此时val是一个所有权变量，不加&，val就获得了所有权
    for val in nums {
        println!("val:{}", val);
    }

    // 下面的语句报错，因为nums已经在上面的迭代中，消费完毕，也就是每个元素的所有权val已经drop了
    // println!("nums:{:?}", nums); // 这一句就会报错 value borrowed here after move
}

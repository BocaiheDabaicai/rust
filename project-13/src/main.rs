fn main() {
    // 1. 创建向量
    let _array_1: [&str; 3] = ["sad", "happy", "gray"]; // 传统数组是一种固定大小、固定类型的数据结构

    let array_2: Vec<i32> = Vec::new(); // 方式一
    // let array_2 = Vec::<i32>::new(); // 方式二
    println!("{:?}", &array_2);

    let array_3: Vec<&str> = Vec::new();
    println!("{:?}", &array_3);

    let array_4 = vec![1, 2, 3, 4, 5]; // 该方式支持类型判断
    println!("{:?}", &array_4);

    // 2. 增加元素、删除元素
    let mut array_5 = vec![21.1, 3.2, 5.0, 6.2, 12.3];
    println!("Now, {:?}", &array_5);

    array_5.push(7.4);
    println!("After push, {:?}", &array_5);

    array_5.insert(2, 7.4);
    println!("After insert 2, {:?}", &array_5);

    array_5.insert(0, 7.4);
    println!("After insert 0, {:?}", &array_5);

    let option_1 = array_5.pop(); // pop 后得到一个可选枚举值
    println!("After pop, {:?}", &array_5);
    println!("option_1: {:?}", &option_1.unwrap_or(0.0));

    array_5.remove(0);
    println!("After remove 0, {:?}", &array_5);

    array_5.remove(5);
    println!("After remove 5, {:?}", &array_5);

    // 3. 读取元素
    let array_6 = vec!["qee", "gdr", "wrr", "ddz", "qwh"];
    println!("Now: {:?}", &array_6);

    println!("Get data 0 with immediately: {:?}", &array_6[0]); // 直接获得值，本处取的是字符引用
    println!("Get data 3 with immediately: {:?}", &array_6[3]);
    println!("Get data 4 with immediately: {:?}", &array_6[4]);

    println!("Get data with area [0..2]: {:?}", &array_6[0..2]); // 获得子数组
    println!("Get data with area [2..4]: {:?}", &array_6[2..5]);

    println!("Get data 0 with function: {:?}", array_6.get(0)); // get 后获得一个可选枚举值
    println!("Get data 3 with function: {:?}", array_6.get(3));
    println!(
        "Get data 4 with function: {:?}",
        array_6.get(4).unwrap_or(&"error")
    ); // 包含错误处理的解构处理

    // 4. get方法读取元素
    // 避免了索引不存在的问题

    match array_6.get(2) {
        None => println!("None is happen."),
        Some(value) => println!("Get data 3 with function: {:?}", value),
    }

    match array_6.get(7) {
        None => println!("None is happen."),
        Some(value) => println!("Get data 3 with function: {:?}", value),
    }

    // 5. 向量的所有权
    /*
        可以存在多个不可变引用和一个可变引用
        在创建一个可变引用后，原变量无法执行 push 等修改向量内部的操作
    */
    let array_7 = vec![
        String::from("djuue388"),
        String::from("ogipp842"),
        String::from("pohlk112"),
    ];
    let reference_1 = &array_7[0];
    let reference_2 = &array_7[0];

    println!("reference_1: {:?}", &reference_1);
    println!("reference_2: {:?}", &reference_2);

    // 6. 写入向量元素
    let mut array_8 = vec![
        String::from("djuue388"),
        String::from("ogipp842"),
        String::from("pohlk112"),
    ];

    // let copy_1 = array_8;

    array_8.push(String::from("ghurh664"));

    let reference_3 = &mut array_8[0];
    reference_3.push_str(" and hjjui998");

    println!("reference_3: {:?}", &reference_3);

    println!("Final Array is: {:?}", array_8);

    // 7. 向量幕后的工作流程
    let mut seasons = Vec::<&str>::with_capacity(4);

    println!("seasons length is: {}", seasons.len());
    println!("seasons capacity is: {}", seasons.capacity());
    println!("seasons data is: {:?}", seasons);

    seasons.push("Spring");
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");

    println!("seasons length is: {}", seasons.len());
    println!("seasons capacity is: {}", seasons.capacity());
    println!("seasons data is: {:?}", seasons);

    seasons.push("Other"); // 向量支持容量扩展

    println!("seasons length is: {}", seasons.len());
    println!("seasons capacity is: {}", seasons.capacity());
    println!("seasons data is: {:?}", seasons);
}

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
}

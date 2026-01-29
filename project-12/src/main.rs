fn main() {
    // 1. 可选枚举
    let data_enum_1 = Option::Some(5);
    let data_enum_2 = Option::Some("hello");
    let data_enum_3 = Option::Some(true);
    let data_enum_4 = Option::Some("qqweea".to_string());
    let data_enum_5: Option<i16> = Option::Some(12); // 同质表达 - 1
    let data_enum_6 = Option::<i16>::Some(5); // 同质表达 - 1
    let data_enum_7: Option<&str> = Option::None; // 表达为None时，需要指明变量的类型

    // 2. 可选枚举实例
    let data_enum_8 = [
        String::from("ssa123"),
        String::from("pco312"),
        String::from("ckl321"),
    ];

    let data_enum_9: Option<&String> = data_enum_8.get(2); // 获得字符串引用
    println_self(&data_enum_9);

    let data_enum_10: Option<&String> = data_enum_8.get(12);
    println_self(&data_enum_10);

    // 3. 解包和期待的方法
    let data_enum_11 = data_enum_9.unwrap(); // 解包裹方法
    println_self(&data_enum_11);

    // let data_enum_12 = data_enum_10.unwrap();    // None 值无法解除捆绑
    // println_self(&data_enum_12);

    let data_enum_13 = data_enum_9.expect("Have no get a value."); // 期待值会进行解包处理，但在出错时打印文字描述
    println_self(&data_enum_13);

    // let data_enum_14 = data_enum_10.expect("Have no get a value."); // 期待值会进行解包处理，但在出错时打印文字描述
    // println_self(&data_enum_14);
}

fn println_self<T: std::fmt::Debug>(value: &T) {
    println!("{:?}", value);
}

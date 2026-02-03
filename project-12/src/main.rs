// 8. 源头构建 Option 对象 - part1
#[derive(Debug, Copy, Clone)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn unwarp(self) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Have no value."),
        }
    }

    fn unwarp_or(self, fallback_value: T) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

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

    // 4. 枚举匹配关键字
    let data_enum_14: Option<&String> = data_enum_8.get(2);

    match data_enum_14 {
        Option::Some(instrument) => println!("Match Result: {:?}", instrument),
        Option::None => println!("Match Result: None"),
    }

    play(data_enum_8.get(1));
    play(data_enum_8.get(12));

    // 5. 从函数中返回枚举
    let availability_1 = is_item_in_stock(true, true);
    let availability_2 = is_item_in_stock(true, false);
    let availability_3 = is_item_in_stock(false, false);

    println_self(&availability_1);
    println_self(&availability_2);
    println_self(&availability_3);

    match availability_1 {
        // Some(value) => println!("Func Match Result: {:?}", value),  // 收集所有值
        Some(true) => println!("Func Match Result: true"),
        Some(false) => println!("Func Match Result: false"),
        None => println!("Func Match Result: None"),
    }

    // 6. 顶级可选枚举
    /*
        简写方式：
        - Option::Some() -> Some()
        - Option::None -> None
    */

    // 7. 解构或方法
    let availability_4 = Some(12);
    let availability_5: Option<i32> = None; // 为 None 指明数据类型

    println!("{}", availability_4.unwrap_or(0));
    println!("{}", availability_5.unwrap_or(1));

    // 8. 源头构建 Option 对象
    let availability_6 = MyOption::Some(99);
    let availability_7: MyOption<i32> = MyOption::None;

    println!("{}", availability_6.unwarp());
    // println!("{}",availability_7.unwarp()); // 因为没有数值，将引起恐慌
    println!("{}", availability_7.unwarp_or(32));

    // 9. 结果枚举
    let data_enum_15: Result<i32, &str> = Result::Ok(12);
    let data_enum_16: Result<i32, &str> = Result::Err("Something is wrong.");

    // println!("{data_enum_15:?}");
    // println!("{:?}",data_enum_15);

    println_self(&data_enum_15);
    println_self(&data_enum_16);

    // 10. 结果枚举实例
    let text = "50";
    let text_number = text.parse::<i32>();

    println!("{:?}", &text_number);

    // 11. 从函数中返回结果枚举
    // 本处我进行了解包以及错误处理
    println!("divide result is: {:?}", divide(12.0, 4.0).unwrap_or(1.0));
    println!("divide result is: {:?}", divide(12.0, 0.0).unwrap_or(0.0));

    let result_enum_1 = divide(13.4, 1.2);

    match result_enum_1 {
        Ok(val) => println!("the Result is {}", val),
        Err(message) => println!("Func Match Result: {:?}", message),
    }

    // 12. 结果方法
    // 参见 11
}

fn println_self<T: std::fmt::Debug>(value: &T) {
    println!("{:?}", value);
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Func Match Result: {:?}", instrument),
        Option::None => println!("Func Match Result: None"),
    }
}

fn is_item_in_stock(item_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

// 11. 从函数中返回结果枚举 - part1
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Denominator is zero."))
    } else {
        Ok(numerator / denominator)
    }
}

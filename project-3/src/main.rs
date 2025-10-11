use std::any::Any;

#[allow(unused_variables)]
fn main() {
    let data1: i8 = 127;
    let data2: u8 = 255;

    let data3: i16 = -30000;
    let data4: i16 = 30000;

    let data5 = 25i8;
    let data6 = 25u8;

    let data7 = 25_32_1; // 下划线表示，最终的值会消除下划线
    println!("{data7}");

    // 根据计算机操作系统的位，来分配变量的空间大小，方便跨系统使用变量
    let data8: isize = 25;
    let data9: usize = 25;

    // 字符串及特殊字符的表示方法
    let data10: &str = "C:\\MyDocument\\new\\video.mp4";
    let data11: &str = "This is my apple,\nnot yours.";
    let data12: &str = "\tHello! Is there somebody here?";
    println!("{data10}");
    println!("{data11}");
    println!("{data12}");

    // 变量的内部函数方法
    let data13: i16 = -24;
    println!("{}", data13.abs());
    println!("{}", data13.pow(2));
    let data14: &str = "  dqwdxxx text ";
    println!("{}", data14.trim());

    // 浮点数，位数是对精度的控制
    let data15: f32 = 7.141592653589793;
    let data16: f64 = 7.141592653589793;
    let data17: f32 = 10.56;
    let data18: f32 = 10.46;
    println!("{}", data15);
    println!("{}", data16);
    println!("{}", data17.floor()); // 向下取整
    println!("{}", data17.ceil()); // 向上取整
    println!("{}", data17.round()); // 四舍五入
    println!("{}", data18.round()); // 四舍五入

    // 自定义描述符
    let data19: f32 = 10.46252;
    println!("{data19:.2}");

    // 数据类型强制转换
    let data20: i8 = 12;
    let data21: i32 = data20 as i32;
    let data22: f32 = 24.12;
    let data23: i8 = data22 as i8;
    println!("{data21} {:?}", data21.type_id());
    println!("{data23} {:?}", data23.type_id());

    // 算术表达式
    let add: f32 = 2.1 + 3.2;
    let sub: f32 = 2.1 - 3.2;
    let mul: f32 = 2.1 * 3.2;
    let div: f32 = 2.1 / 3.2;
    let mol: f32 = 9.1 % 3.2;
    println!("add: {add:.2}");
    println!("sub: {sub:.2}");
    println!("mul: {mul:.2}");
    println!("div: {div:.2}");
    println!("mol: {mol:.2}");

    // 赋值表达
    let mut data24: i8 = 8;
    println!("Now, I have {} apples.", data24);
    data24 += 1;
    println!("Add 1, I have {} apples.", data24);
    data24 -= 1;
    println!("Sub 1, I have {} apples.", data24);
    data24 *= 2;
    println!("Mul 2, I have {} apples.", data24);
    data24 /= 2;
    println!("Div 2, I have {} apples.", data24);

    // 布尔类型
    let is_beautiful: bool = true;
    println!("I have a {} face.", is_beautiful);
    println!("I have a {} face.", !is_beautiful);

    let age: i8 = 32;
    let is_young: bool = age > 18;
    println!("{}", is_young);
    println!("{} {}", age.is_negative(), age.is_positive());
    println!("I'm {} years old, can I go to a cinema? {}", age, is_young);

    // 逻辑表达式
    println!("\"Coke\" == \"Coke\", result is: {}", "Coke" == "Coke");
    println!("\"Coke\" == \"coke\", result is: {}", "Coke" == "coke");
    println!("\"Coke\" != \"Coke\", result is: {}", "Coke" != "Coke");
    println!("\"Coke\" != \"coke\", result is: {}", "Coke" != "coke");

    let data26: bool = true;
    let data27: bool = true;
    let data28: bool = false;
    let data29: bool = data26 && data27;
    let data30: bool = data26 && data27 && data28;
    println!("data29:bool = data26 && data27, result is: {}", data29);
    println!(
        "data30:bool = data26 && data27 && data28, result is: {}",
        data30
    );

    let data31: bool = true;
    let data32: bool = false;
    let data33: bool = false;
    let data34: bool = data31 || data32;
    let data35: bool = data31 || data32 || data33;
    let data36: bool = data31 || data32 || data33 && data34;
    println!("data34:bool = data31 || data32, result is: {}", data34);
    println!(
        "data35:bool = data31 || data32 || data33, result is: {}",
        data35
    );
    println!(
        "data36:bool = data31 || data32 || data33 && data34, result is: {}",
        data36
    );

    // 字符格式
    let data37: char = 'B';
    let data38: char = '🍔';
    println!(
        "char is: {}, Is a alphabetic(): {}, is_uppercase(): {}, is_lowercase(): {}",
        data37,
        data37.is_alphabetic(),
        data37.is_uppercase(),
        data37.is_lowercase()
    );
    println!("char is: {}", data38);

    // 数组
    let numbers: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // 数组是固定大小的元素，不能多也不能少
    let mut apples: [&str; 4] = ["asd", "dq12", "ffgr", "dewq441"];
    println!("apples.len: {}", apples.len());
    let empty_array: [f32; 0] = [];

    let index_1: &str = apples[0];
    let index_2: &str = apples[1];
    let index_3: &str = apples[2];
    let index_4: &str = apples[3];
    println!("index_1: {}", index_1);
    println!("index_2: {}", index_2);
    println!("index_3: {}", index_3);
    println!("index_4: {}", index_4);

    apples[0] = "ds453453";

    println!("apples[0]: {}", apples[0]);
    println!("apples[1]: {}", apples[1]);
    println!("apples[2]: {}", apples[2]);
    println!("apples[3]: {}", apples[3]);

    // trait
    let seasons: [&str; 6] = ["summer", "fall", "winter", "spring", "autumn", "spring"];
    // println!("{}", seasons);    // 这是无法直接打印的
    println!("seasons: {:?}", seasons); // 方式一
    println!("seasons: {seasons:?}"); // 方式二
    println!("seasons: {:#?}", seasons); // 漂亮格式打印数组

    // debug
    dbg!(2 + 3);
    dbg!(seasons);

    // 元组
    let employee = ("Bob", 32, "programming");
    let name = employee.0;
    let age = employee.1;
    let career = employee.2;
    let (name, age, career) = employee; // 获取元组值的另外一种方式

    dbg!(employee);
    dbg!(name);
    dbg!(age);
    dbg!(career);

    // 范围和范围迭代
    let month_days = 1..31; // 表示1-30的范围
    let month_days_two = 1..=31; // 表示1-31的范围
    let letters = 'a'..='x';

    println!("{:?}", month_days);
    println!("{:?}", month_days_two);

    /*
        dbg 被用于一个值后，这个值就不能另作他用，
        如果改变这种情况，用只能通过给dbg引用值的地址
        即：dbg!(&data)
    */
    // dbg!(month_days);
    // dbg!(month_days_two);

    for month_day in month_days {
        print!("{} ", month_day);
    }
    println!();

    for letter in letters {
        print!("{} ", letter);
    }
    println!();

    for season in seasons {
        print!("{} ", season);
    }
}

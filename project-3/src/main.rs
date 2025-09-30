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
    println!("\"Coke\" == \"Coke\", result is: {}","Coke" == "Coke");
    println!("\"Coke\" == \"coke\", result is: {}","Coke" == "coke");
    println!("\"Coke\" != \"Coke\", result is: {}","Coke" != "Coke");
    println!("\"Coke\" != \"coke\", result is: {}","Coke" != "coke");

    let data26:bool = true;
    let data27:bool = true;
    let data28:bool = false;
    let data29:bool = data26 && data27;
    let data30:bool = data26 && data27 && data28;
    println!("data29:bool = data26 && data27, result is: {}", data29);
    println!("data30:bool = data26 && data27 && data28, result is: {}", data30);
}

fn main() {
    // 1. 借用一个字符串对象的切片
    /*
        他们均属于字符串切片，只是范围不同
        action_hero // 第二种创建方式
        first_name
        last_name
    */
    // let action_hero = String::from("Hero BobSunBoy");   // 字符串对象，直接提供字符串
    let action_hero = "Hero BobSunBoy";   // 字符串对象，只提供一个字符串的地址
    // let string_reference = &action_hero[0..4];  // Hero
    // let string_reference = &action_hero[..4];   // Hero
    let string_reference = &action_hero[5..];   // BobSunBoy
    println!("{}", string_reference);

    let first_name = &action_hero[0..4];
    let last_name = &action_hero[5..];
    println!("So you name is: {} {}", first_name,last_name);

    // 2. 块中创建字符串引用，并返回部分字符串切片
    let final_name = {
        // let action_hero = String::from("Hero BobSunBoy");   // 只创建字符串对象，取不到字符串内容
        let action_hero = "Hero BobSunBoy"; // 连同地址引用和部分堆空间，一起分配给变量 final_name
        &action_hero[0..4]
    };

    println!("{}",final_name);
}

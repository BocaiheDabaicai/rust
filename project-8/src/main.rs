fn main() {
    // 1. 借用一个字符串对象的切片
    /*
        他们均属于字符串切片，只是范围不同
        action_hero // 第二种创建方式
        first_name
        last_name
    */
    // let action_hero = String::from("Hero BobSunBoy");   // 字符串对象，直接提供字符串
    let action_hero = "Hero BobSunBoy"; // 字符串对象，只提供一个字符串的地址
    // let string_reference = &action_hero[0..4];  // Hero
    // let string_reference = &action_hero[..4];   // Hero
    let string_reference = &action_hero[5..]; // BobSunBoy
    println!("{}", string_reference);

    let first_name = &action_hero[0..4];
    let last_name = &action_hero[5..];
    println!("So you name is: {} {}", first_name, last_name);

    // 2. 块中创建字符串引用，并返回部分字符串切片
    let final_name = {
        // let action_hero = String::from("Hero BobSunBoy");   // 只创建字符串对象，取不到字符串内容
        let action_hero = "Hero BobSunBoy"; // 连同地址引用和部分堆空间，一起分配给变量 final_name
        &action_hero[0..4]
    };

    println!("{}", final_name);

    // 3. 字符串切片的长度
    let food = "pizza🍕"; // 图标字符占四个字符长度
    println!("Food length: {}", food.len());
    let pizza_slice = &food[0..3];
    println!("pizza_slice length: {}", pizza_slice.len());

    // 4. 字符串语法快捷实现
    let action_origin = String::from("Hero BobSunBoy");

    let first_origin = &action_origin[..5];
    println!("His first name is: {}", first_origin);

    let last_origin = &action_origin[5..];
    println!("His last name is: {}", last_origin);

    let full_origin = &action_origin;
    println!("His full name is: {}", full_origin);

    // 5. 字符串切片与函数参数
    let action_double = String::from("Hero BobSunBoy");
    do_hero_stuff(&action_double);
    let action_double_another = "Hero BobSunBoy";
    do_hero_stuff(action_double_another);
}

fn do_hero_stuff(hero: &str) {
    // hero: &String 仅支持字符串对象引用
    // hero: &str 支持字符串对象引用、字符引用，范围支持上更好
    // 可以将 &String -> &str ，反过来无法实现
    println!("{} saves the day", hero);
}

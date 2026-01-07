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

    // 6. 切片数组
    let values = [49, 82, 63, 50, 33, 21];

    let slice_values = &values[1..3];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values[3..];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values[..3];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values[..];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values;
    println!("slice_values: {slice_values:?}");

    // 7. 数组引用与数组，数组引用的函数定义
    let values = [49, 82, 63, 50, 33, 21];

    let values_reference = &values;
    print_length(values_reference);

    let three_reference = &values[..3];
    print_length(three_reference);

    // 8. 可变的数组切片
    let mut my_array = [12, 3, 2, 41, 66, 7];
    let my_slice = &mut my_array[2..4]; // 数组的可变引用

    println!("My slice is: {:?}", my_slice);
    my_slice[0] = 48;

    println!("My slice is: {:?}", my_slice);
    println!("My array is: {:?}", my_array); // 引用修改数据后，堆空间的值会发生变化

    // 9. test
    let mut cereals = [
        "Cookie Crisp",
        "Cinnamon Toast Crunch",
        "Frosted Flakes",
        "Cocoa Puffs",
        "Captain Crunch",
    ];

    let first_two = &cereals[0..2];
    println!("first_two: {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("mid_three: {:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("last_three: {:?}", last_three);

    last_three[2] = "Lucky Charms";
    println!("cereals: {:?}", cereals);

    let cookie_crisp = String::from("Cookie Crisp");

    let cookie = &cookie_crisp[0..7];
    println!("cookie: {}", cookie);

    let cocoa_puffs = "Cocoa Puffs";

    let puffs = &cocoa_puffs[6..];
    println!("puffs: {}", puffs);
}

fn do_hero_stuff(hero: &str) {
    // hero: &String 仅支持字符串对象引用
    // hero: &str 支持字符串对象引用、字符引用，范围支持上更好
    // 可以将 &String -> &str ，反过来无法实现
    println!("{} saves the day", hero);
}

fn print_length(reference: &[i32]) {
    // 数组或数组引用的描述：reference:[i32;6],reference:&[i32;6]
    // 数组引用的类型描述会更加松解：reference:&[i32]
    println!("the length is: {}", reference.len());
}

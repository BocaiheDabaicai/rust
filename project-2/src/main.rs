fn main() {
    let apples_in_garden = 50;
    let oranges = 21 + 19;
    let fruits = oranges + apples_in_garden;
    let _fruits1 = oranges + apples_in_garden;  // 忽略未使用的变量，增加下划线

    println!("Hello, world!");
    // 输出值的两种方法
    println!("My garden has {} fruits", fruits);
    println!("My garden has {fruits} fruits");
    // 多个变量的输出情况
    println!("{} + {} = {}", apples_in_garden, oranges, fruits);
    println!("{apples_in_garden} + {oranges} = {fruits}");
    // 函数的输出情况
    println!("{}", get_data());
    // println!("{get_data()}", ); // 函数不能直接使用
    // 指定使用哪一个变量
    println!("{0} + {1} = {2}", apples_in_garden, oranges, fruits);
    println!("{0} {0} {1} {2}", apples_in_garden, oranges, fruits);
    println!("{0} {1} {1} {2}", apples_in_garden, oranges, fruits);
    println!("{0} {1} {2} {2}", apples_in_garden, oranges, fruits);
}

fn get_data() -> i32 {
    return 10;
}

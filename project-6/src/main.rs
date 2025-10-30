fn main() {
    // 栈的实现过程
    /*
        result1 先入栈
        result2 后入栈
        result2 先出栈
        result1 后出栈
    */
    let result1: i16 = 12;
    let result2: i16 = 18;

    println!("{}", result1);
    println!("{}", result2);

    // copy trait
    let time = 2025;
    let year = time; // 通过 time 的地址，将 time 的值赋予 year;

    println!("time: {}", time);
    println!("year: {}", year);

    // string 类型
    let food: &str = "pasta";
    let text = String::new();   // 堆数据创建
    let candy = String::from("KitKat Candy");   // 堆数据创建并赋值

}

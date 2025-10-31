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
    let text = String::new(); // 堆数据创建
    let candy = String::from("KitKat Candy"); // 堆数据创建并赋值

    // push
    let mut name = String::from("Jobs");
    name.push_str(" is a good man.");

    println!("{}", name);

    // move
    let person = String::from("Boris");
    let genius = person; // 不会复制 person 的堆数据，此时表示 person 的值被移动到了 genius 上，并无法使用

    // println!("{}", person);

    // drop 释放堆空间
    let person1 = String::from("Boris");

    drop(person1);
    // println!("{}", person1); // 再次使用将无效

    // copy
    let person2 = String::from("Boris");
    let genius2 = person2.clone();  // 克隆一个新的堆空间，把数据交给 genius2

    println!("{}",person2);
}

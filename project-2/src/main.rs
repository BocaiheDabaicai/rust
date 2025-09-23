const TAX_RATES: f64 = 0.075; // 常量声明

fn main() {
    let apples_in_garden = 50;
    let oranges = 21 + 19;
    let fruits = oranges + apples_in_garden;
    let _fruits1 = oranges + apples_in_garden; // 忽略未使用的变量，增加下划线
    let mut rep_gym = 1;
    let _grams_of_protein = "100.32";
    let _grams_of_protein = 100.32;
    let mut grams_of_protein = 100; // 实现变量的重新声明
    let mut coffee_price = 5.99; // 变量使用在函数作用域内，bug
    let income:i32 = 9000; // 声明变量的类型

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

    // 变量默认为不可变，需要添加 mut
    println!("The Data is: {rep_gym}.And the number is immutable.");
    rep_gym = 2;
    println!("The Data is: {rep_gym}.And the number is immutable.");

    // 实现变量的重新声明
    println!("The rechange data is {grams_of_protein}");
    grams_of_protein = 105;
    println!("The rechange data is {grams_of_protein} again");

    {
        /* 此处做了两件事情
        1. 对作用域外的变量进行重新赋值，并输出
        2. 重新声明一个和外部作用域变量名相同的变量，并输出
        结论：
        1. 可以对外部变量进行重新赋值
        2. 可以声明与外部变量相同的变量
        3. 内部作用域会优先使用内部的变量，如果没有找到，就会去外部的作用域寻找变量
        4. 这里应该是一个Bug，因为我在外部读取了该数值 coffee_price
         */
        coffee_price = 2.1; // 支持内部作用域访问外部作用域
        println!("The coffee_price is {coffee_price}");
        let coffee_price = 7.99; // 变量使用在函数作用域内，且重新声明
        println!("The coffee_price is {coffee_price}");
    }
    coffee_price = 2.11;
    println!("The coffee_price is {coffee_price}");


    println!("My income is {income}.The const_data is {TAX_RATES}");
}

// coffee_price = 2; // 超过了函数的作用域

fn get_data() -> i32 {
    return 10;
}

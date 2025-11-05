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
    let _food: &str = "pasta";
    let _text = String::new(); // 堆数据创建
    let _candy = String::from("KitKat Candy"); // 堆数据创建并赋值

    // push
    let mut name = String::from("Jobs");
    name.push_str(" is a good man.");

    println!("{}", name);

    // move
    let person = String::from("Boris");
    let _genius = person; // 不会复制 person 的堆数据，此时表示 person 的值被移动到了 genius 上，并无法使用

    // println!("{}", person);

    // drop 释放堆空间
    let person1 = String::from("Boris");

    drop(person1);
    // println!("{}", person1); // 再次使用将无效

    // copy
    let person2 = String::from("Boris");
    let _genius2 = person2.clone(); // 克隆一个新的堆空间，把数据交给 genius2

    println!("{}", person2);

    // reference
    let my_ref = 12;
    let my_ref_ref = &my_ref; // 指向一个地址
    println!("my_ref: {}", my_ref);
    println!("&my_ref: {}", &my_ref);
    println!("my_ref_ref: {}", my_ref_ref);
    println!("*my_ref_ref: {}", *my_ref_ref);

    println!("----");

    let my_ref_str = String::from("Boris"); // 字符串
    let my_ref_str_ref = &my_ref_str; // 指向一个地址
    println!("my_ref_str: {}", my_ref_str);
    println!("&my_ref_str: {}", &my_ref_str);
    println!("my_ref_str_ref: {}", my_ref_str_ref);
    println!("*my_ref_str_ref: {}", *my_ref_str_ref);

    println!("----");

    let my_ref_str_ice = "Cookies and Cream"; // 字符串引用
    let my_ref1 = my_ref_str_ice; // 字符串引用
    let my_ref2 = &my_ref_str_ice; // 字符串引用的引用
    println!("my_ref1: {}", my_ref1);
    println!("my_ref2: {}", my_ref2);

    // 引用函数
    let my_ref3 = 6;
    let my_ref4 = String::from("Cookies and Cream");
    print_my_value(my_ref3);
    print_my_value_string(my_ref4);
    // println!("{my_ref4}"); // 不可以再次引用，因为值已经被先使用了

    // 可变参数
    let burger = String::from("burger");
    add_fries(burger);

    // 函数返回值
    let cake = back_cake();
    println!("I have a {} cake now.", cake);

    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);
    current_meal = add_salt(current_meal);

    println!("{}",current_meal);

    // test
    let is_concert = false;
    let is_event = is_concert;
    // 所有权没有转移
    println!("is_concert: {}", is_concert);
    println!("is_event: {}", is_event);

    let sushi = "Salmon";
    let dinner = sushi;
    // 所有权没有转移
    println!("sushi: {}", sushi);
    println!("dinner: {}", dinner);

    let sushi2 = String::from("Salmon");
    let dinner2 = sushi2;
    // 所有权发生转移
    // println!("sushi2: {}", sushi2);
    println!("dinner2: {}", dinner2);

    let mut clear_str = String::from("clearclear");

    println!("Before clear_str: {}",clear_str);
    clear_str.clear();
    println!("After clear_str: {}",clear_str);

    let clear_str2 = String::from("clearclear2");
    eat_meal(clear_str2);

    let mut clear_str3 = String::from("We have a cup of apples.");
    clear_str3 = eat_meal_return(clear_str3);
    println!("now clear_str3 is empty: {}",clear_str3);
}

fn print_my_value(value: i32) {
    println!("value is: {}", value);
}

fn print_my_value_string(value: String) {
    println!("value is: {}", value);
}

fn add_fries(mut meal: String) {
    meal.push_str(" and fries");

    println!("{}", meal);
}

fn back_cake() -> String {
    // 方式一
    /*let cake = String::from("Cookies and Cream");

    return cake;*/

    // 方式二
    // return String::from("Cookies and Cream");

    // 方式三
    String::from("Cookies and Cream")
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour ");
    return meal
}

fn add_sugar(mut meal: String)-> String{
    meal.push_str("Add sugar ");
    return meal
}
fn add_salt(mut meal: String)-> String{
    meal.push_str("Add salt ");
    return meal
}

fn eat_meal(mut meal:String){
    println!("Before meal: {}",meal);
    meal.clear();
    println!("After meal: {}",meal);
}

fn eat_meal_return(mut meal:String) -> String {
    println!("Before meal: {}",meal);
    meal.clear();
    println!("After meal: {}",meal);

    return meal;
}

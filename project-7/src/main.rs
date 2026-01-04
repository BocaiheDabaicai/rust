fn main() {
    // 1. 向函数中传递字符串对象的可修改引用
    let mut current_meal: String = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // 2. 对象引用可以共存，对象引用且修改完毕之后，才能被再次借用
    let mut car = String::from("Red");
    let ref1 = &mut car; // 引用可修改字符串
    ref1.push_str(" Add carrot");
    println!("ref1: {}", ref1);
    let ref2 = &car;

    // println!("&car: {}", &car);
    // println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);

    // 3. 获取字符串引用，在上一个修改引用用完之后，才能将修改引用借给下一个变量
    /*let coffee: String = String::from("Mocha");
    let a = &coffee;
    let b = a;
    println!("a: {}", a);
    println!("b: {}", b);*/

    let mut coffee: String = String::from("Mocha");
    let a = &mut coffee;
    println!("a: {}", a);
    let b = a;
    println!("b: {}", b);

    // 4. 空引用
    let reference = create_city();
    println!("reference: {}", reference);

    // 5. 对数组和元组进行引用
    /*let registrations = [true,true,false];    // 数组方法
    let first = registrations[0];*/
    let registrations = (true, true, false); // 元组方法
    let first = registrations.0;
    println!("first: {}, registrations:{registrations:?}", first);

    /*let languages = [ // 数组方法
        String::from("Rust"),
        String::from("JavaScript")
    ];
    // let first = languages[0].clone(); // 不会移动字符串的所有权，只是文本内容不会是原文本
    let first = &languages[0]; // 对原字符串对象进行引用，获取字符串地址*/
    let languages = (
        // 元组方法
        String::from("Rust"),
        String::from("JavaScript"),
    );
    // let first = languages.0.clone(); // 对原字符串对象进行引用，获取字符串地址
    let first = &languages.0; // 对原字符串对象进行引用，获取字符串地址
    println!("first: {}, languages:{languages:?}", first);

    // test
    let mut trip = start_trip();
    println!("trip: {}", trip);
    trip.push_str(" And ");
    add_city_one(&mut trip);
    add_city_two(&mut trip);
    add_city_three(&mut trip);
    show_itinerary(&trip);
}

// meal: String
// mut meal : String
// meal : &String，传递引用
// meal : &mut String，传递应用且可以修改

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("meal: {}", meal);
}

/*fn create_city() -> &String {
    // 悬空引用（空指针）
    // 当变量超出作用域时，就会被内存回收处理
    let city = String::from("Shanghai");
    &city
}*/

fn create_city() -> String {
    // 可以改变为直接返回字符串
    String::from("Shanghai")
}

fn start_trip() -> String {
    String::from("Shanghai")
}

fn add_city_one(trip:&mut String) {
    trip.push_str("BeiJing");
    trip.push_str(" And ");
}

fn add_city_two(trip:&mut String) {
    trip.push_str("BeiJing");
    trip.push_str(" And ");
}

fn add_city_three(trip:&mut String) {
    trip.push_str("HongKong");
    trip.push_str(" And ");
}

fn show_itinerary(trip: &String){
    println!("This plan is ... {}",trip);
}
fn main() {
    // 1. 向函数中传递字符串对象的可修改引用
    let mut current_meal: String = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // 2. 对象引用可以共存，对象引用且修改完毕之后，才能被再次借用
    let mut car = String::from("Red");
    let ref1 = &mut car;    // 引用可修改字符串
    ref1.push_str( " Add carrot");
    println!("ref1: {}", ref1);
    let ref2 = &car;

    // println!("&car: {}", &car);
    // println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
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

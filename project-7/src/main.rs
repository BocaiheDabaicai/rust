fn main() {
    // 1. 向函数中传递字符串对象的可修改引用
    let mut current_meal: String = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);
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

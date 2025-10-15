fn main() {
    // 函数、函数传参、函数返回值
    open_store("Bob Steven");
    bake_pizza(8, "pepperoni");
    swim_in_profit();
    dbg!(square(6));
    dbg!(square_two(8));

    // 元组是一种函数返回值
    let _result = ();
    let _result_1 = mystery();

    // 作用域返回值
    let multiplier = 2;
    let calculation = {
        let value = 3 * 3;
        value * multiplier
    };
    println!("calculation = {}",calculation);

    // test
    apply_to_jobs(35, "Rust Developer");
    dbg!(is_even(8));
    dbg!(is_even(9));
    println!("{:?}", alphabets("aardvark")); // -> (true, false)
    println!("{:?}", alphabets("zoology"));  // -> (false, true)
    println!("{:?}", alphabets("zebra"));    // -> (true, true)
}

fn open_store(neighborhood: &str) {
    println!("Opening store: {}", neighborhood);
}

fn bake_pizza(number: i16, topping: &str) {
    println!("Baking {} {} pizzas.", number, topping);
}

fn swim_in_profit() {
    println!("Swimming in profit");
}

fn square(number: i32) -> i32 {
    return number * number;
}

fn square_two(number: i32) -> i32 {
    println!("{}", square(6));
    number * number
}

fn mystery(){
    println!("Hello there.");
}

fn apply_to_jobs(number:i32,title:&str){
    println!("I'm applying to {} {} jobs.",number,title);
}

fn is_even(number:i32) -> bool{
    return number % 2 == 0;
}

fn alphabets(text:&str) -> (bool,bool){
    return (text.contains('a'), text.contains('z'));
}
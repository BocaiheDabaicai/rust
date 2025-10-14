fn main() {
    open_store("Bob Steven");
    bake_pizza(8, "pepperoni");
    swim_in_profit();
    dbg!(square(6));
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

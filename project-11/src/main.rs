// 1. 泛型介绍 - part2
#[derive(Debug)]
struct DeliSwitch {}

#[derive(Debug)]
enum Option {
    Add,
    Bcc,
}

fn main() {
    // 1. 泛型介绍
    println_self(identity(1));
    println_self(identity(1.3));
    println_self(identity("hello 11234"));
    println_self(identity(String::from("hello")));
    println_self(identity(true));
    println_self(identity(DeliSwitch {}));
    println_self(identity(Option::Add));
}

// 1. 泛型介绍 - part1
/*
    泛型类型名称可以自定义，一般使用 T
*/
fn identity<T>(value: T) -> T {
    value
}

fn println_self<T: std::fmt::Debug>(data: T) {
    println!("{:?}", data);
}

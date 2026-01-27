// 1. 泛型介绍 - part2
#[derive(Debug)]
struct DeliSwitch {}

#[derive(Debug)]
enum Option {
    Add,
    Bcc,
}

// 4. 数据结构中的泛型 - part1
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

// 5. 接入模块中的泛型 - part1
impl TreasureChest<String> {
    fn clean_treasure(self: &mut Self) {
        // self.treasure = String::from("Toast");
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_treasure(self: &Self) -> usize {
        // self.treasure = String::from("Toast");
        self.treasure.len()
    }
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

    // 2. 泛类操作符
    println_self(identity::<i8>(1));
    println_self(identity::<i16>(1));
    println_self(identity::<u32>(1));
    println_self(identity::<f32>(1.0));
    println_self(identity::<&str>("1.0"));
    println_self(identity::<String>(String::from("1.--0")));
    println_self(identity::<DeliSwitch>(DeliSwitch {}));
    println_self(identity::<Option>(Option::Bcc));

    // 3. 多泛型
    println_self(make_tuple(12, 3));
    println_self(make_tuple("12", 3));
    println_self(make_tuple(String::from("12"), 3));

    println_self(make_tuple_two(32, 3));

    println_self(make_tuple_three(String::from("12"), 3));

    // 4. 数据结构中的泛型
    let mut treasure_chest_1 = TreasureChest {
        captain: String::from("TreasureChest"),
        treasure: String::from(" Gold  "),
    };
    let treasure_chest_2 = TreasureChest {
        captain: String::from("TreasureChest"),
        treasure: ["Sliver", "Brown", "Brown"],
    };

    // println_self(treasure_chest_1);
    // println_self(treasure_chest_2);

    // 5. 接入模块中的泛型
    println!("treasure_chest_1.treasure: {}",&treasure_chest_1.treasure);
    treasure_chest_1.clean_treasure();
    println!("treasure_chest_1.treasure: {}",&treasure_chest_1.treasure);

    println_self(treasure_chest_2.amount_treasure());
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

// 3. 多泛型 - part1
fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

// 要求 first, second 为同一个T类型
fn make_tuple_two<T>(first: T, second: T) -> (T, T) {
    (first, second)
}

// 要求 first, second 为各自的类型，可以相同、也可以不相同
fn make_tuple_three<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

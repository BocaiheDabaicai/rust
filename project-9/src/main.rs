// 1. 数据结构定义
struct Coffee {
    // 命名规则：各单词首字母大写
    name: String,
    price: f64,
    is_available: bool,
}

fn main() {
    // 2. 创建数据结构实例
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 5.0,
        is_available: true,
    };

    // 3. 读取数据结构的属性
    println!("-- mocha --");
    println!("name: {}", mocha.name);
    println!("price: {}", mocha.price);
    println!("is_available: {}", mocha.is_available);

    let name = mocha.name; // 会转移数据结构对象中 name 的所有权

    println!("Convert name: {}", name);
    // println!("name: {}", mocha.name); // 此处不可运行

    // 4. 复写数据结构的领域
    let mut mocha_great = Coffee {
        // 数据结构只能是全部可修改或者全部不可修改
        name: String::from("Mocha"),
        price: 5.0,
        is_available: true,
    };

    mocha_great.name = String::from("Brushes");

    println!("-- mocha_great --");
    println!("name: {}", mocha_great.name);
    println!("price: {}", mocha_great.price);
    println!("is_available: {}", mocha_great.is_available);

    // 5. 创建一个返回数据结构的函数
    /*
        name 所有权转移过程：
        （1）转移到函数 make_coffee
        （2）转移到函数创建的数据结构中 Coffee
        （3）转移给数据结构对象 mocha_create
    */
    let name = String::from("Apples");
    let mocha_create = make_coffee(name, 21.19, false);

    println!("-- mocha_create --");
    println!("name: {}", mocha_create.name);
    println!("price: {}", mocha_create.price);
    println!("is_available: {}", mocha_create.is_available);

    // 6. 数据结构简洁实例化方法
    let name = String::from("Pears");
    let price = 13.39;
    let is_available = true;
    let mocha_shorthand = Coffee {
        name,
        price,
        is_available,
    };

    println!("-- mocha_shorthand --");
    println!("name: {}", mocha_shorthand.name);
    println!("price: {}", mocha_shorthand.price);
    println!("is_available: {}", mocha_shorthand.is_available);

    // 7. 数据结构更新语法
    let mocha_camel = Coffee {
        name: String::from("Camel Coffee"),
        ..mocha_shorthand // 引用该对象复制它 除name 以外 的所有属性进行使用
    };

    /*let mocha_camel = Coffee {
        ..mocha_shorthand   // 会将 该对象的属性值及所有权全部转移
    };*/

    /*let mocha_camel = Coffee {
        name:mocha_shorthand.name.clone(), // 修正方法
        ..mocha_shorthand
    };*/

    println!("-- mocha_camel --");
    println!("name: {}", mocha_camel.name);
    println!("price: {}", mocha_camel.price);
    println!("is_available: {}", mocha_camel.is_available);

    /*println!("-- mocha_shorthand --");
    println!("name: {}", mocha_shorthand.name);
    println!("price: {}", mocha_shorthand.price);
    println!("is_available: {}", mocha_shorthand.is_available);*/

    // 8. 传递数据结构给函数
    let mut mocha_function = make_coffee(String::from("Popping Coffee"), 21.19, false);
    // drink_coffee(mocha_function);
    // drink_coffee_two(&mocha_function);
    drink_coffee_three(&mut mocha_function);

    /*println!("-- mocha_function --");   // 所有权已经被转移，后续无法使用
    println!("name: {}", mocha_function.name);
    println!("price: {}", mocha_function.price);
    println!("is_available: {}", mocha_function.is_available);*/
}

fn make_coffee(name: String, price: f64, is_available: bool) -> Coffee {
    // 写法一
    /*Coffee {
        name: name,
        price: price,
        is_available: is_available,
    }*/
    // 写法二
    Coffee {
        name,
        price,
        is_available,
    }
}

fn drink_coffee(mut coffee: Coffee) {
    // 数据结构对象本身不需要标注 mut
    println!("-- coffee function --");
    println!("name: {}", coffee.name);
    println!("price: {}", coffee.price);
    println!("is_available: {}", coffee.is_available);

    coffee.is_available = true;
    println!("Config is_available: {}", coffee.is_available);
}

fn drink_coffee_two(coffee: &Coffee) {
    // 不可变引用
    println!("-- coffee function --");
    println!("name: {}", coffee.name);
    println!("price: {}", coffee.price);
    println!("is_available: {}", coffee.is_available);
}

fn drink_coffee_three(coffee: &mut Coffee) {
    // 可变引用
    println!("-- coffee function --");
    println!("name: {}", coffee.name);
    println!("price: {}", coffee.price);
    println!("is_available: {}", coffee.is_available);

    coffee.is_available = true;
    println!("Config is_available: {}", coffee.is_available);
}

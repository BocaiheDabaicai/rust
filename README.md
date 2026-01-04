## Rust

---

#### 安装

- Windows环境
  
  - 配置国内镜像【在系统环境变量里配置】，参考清华镜像源
    
    ```context
    变量名：RUSTUP_DIST_SERVER
    变量值：https://mirrors.tuna.tsinghua.edu.cn/rustup
    
    变量名：RUSTUP_UPDATE_ROOT
    变量值：https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
    ```
  
  - 安装visiual studio installer，配置c++ 桌面开发集成，全选默认安装，大概安装11GB的工具文件
  
  - 下载 rust 安装器，安装rust即可

- 额外
  
  - 配置项目资源镜像【在`config.toml`文件中配置，没有该文件就添加】，可以选择配置**索引版本**和**稀疏索引版本**。【稀疏索引版本可以按需拉取资源，速度更快，支持1.68以上版本；传统索引版本拉取整个版本资源，速度很慢】`参考清华镜像源`
    
    - 配置的位置：`%USERPROFILE%\.cargo\config.toml`，不是项目文件
    
    ```toml
    [source.crates-io]
    replace-with = 'mirror'
    
    [source.mirror]
    registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
    
    [registries.mirror]
    index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
    ```

第三项配置需要移除，此为自定义注册配置，与下载依赖无关，会干扰资源下载。【移除，会影响`cargo search`、`cargo info` 等命令】

#### 命令工具

- rustc，rust编译器
  
  - rustc <fileName>，编译.rc文件，使得访问它可以直接执行

- rustup，rust管理器
  
  - rustup update，更新`rust`官方代码版本，达到最新

- cargo，rust项目管理器
  
  - cargo new <Name>，创建项目
  
  - cargo build，发布可调式版本代码
  
  - cargo build --release，发布正式版本代码
  
  - cargo clean，清理所有发布代码
  
  - cargo run，生成可调式版本代码，并运行
  
  - cargo run --quiet，生成可调式版本代码，并运行，不显示内部构建代码
  
  - cargo check，检查当前代码是否有编译上的问题，并生成可调式代码

- rustfmt，rust代码整理器
  
  - rustfmt <fileName>，格式化该文件的`rust`代码

#### 变量

- 变量声明，`let a` 
- 变量名，**小写，用短横线隔开**，例如：`let last_money_cost_in_hotel:i8 = 2`
- 变量可变性，默认声明的变量都不可变，想要改变就需要添加`mut`，例如`let mut data = 1;`
- 变量声明覆盖，重新声明名字相同的变量，将使最新声明的变量有效，之前的变量统统无效
- 变量作用域，内部作用域可以找外部作用域下的变量进行使用

#### 常量

- 变量声明，`const a:f64 = 7.25`，需要声明数据类型

#### 补充

- 自定义类型，`type <name> = i32...`

- 编译器指令，对检查判断进行改变，可作用于单行代码，也可以作用于整个函数，示例：`#[warn(unused_assignments)]`

##### 代码区

```rust
/*
善意警告：value assigned to `coffee_price` is never read
发生在变量初值未使用，而后面使用了重新赋的值，就会触发该警告
其目的在于提示我可能出现了不必要地赋初值过程
直接使用赋值，或者使用初值，都可以解决这个问题
*/

#![warn(unused_assignments)]  // 作用于整个函数的编译指令，并进行全局强调

type Meters = i32; // 声明自定义数据类型

const TAX_RATES: f64 = 0.075; // 常量声明
const TOUCHDOWN_POINTS:i32 = 6;

fn main() {
    let apples_in_garden = 50;
    let oranges = 21 + 19;
    let fruits = oranges + apples_in_garden;
    let _fruits1 = oranges + apples_in_garden; // 忽略未使用的变量，增加下划线
    let mut rep_gym = 1;
    let _grams_of_protein = "100.32";
    let _grams_of_protein = 100.32;
    let mut grams_of_protein = 100; // 实现变量的重新声明
    let mut coffee_price = 5.99; // 变量使用在函数作用域内
    let income: i32 = 9000; // 声明变量的类型
    let meter: Meters = 10; // 使用自定义数据类型
    #[allow(unused_variables)] // 关闭警告的编译指令
    let two_meter: Meters = 10;

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

    println!("The data is {meter}");

    // mission
    let season:&str = "Summer";
    let mut points_scored:i32 = 28;
    let event_time:&str = "06:00";
    let event_time:i32 = 6;
    let favorite_beverage:&str = "cole";
    points_scored = 35;
    println!("{season} has {points_scored} scores in {event_time}:00pm");
}

// coffee_price = 2; // 超过了函数的作用域

fn get_data() -> i32 {
    return 10;
}
```

---

#### 数据类型

- 有符号整型，符号`i`，支持存储正数、负数、零

- 无符号整型，符号`u`，支持存储正数、零，通常存储更大的正数

- 浮点数，符号`f`，有符号`f32`,`f64`

- 计算机最小的存储单位：字，每8个字叫1个字节

- 整型的取值范围大小：8,16,32,64,128，`i8`表示变量取整型，占8位

- 变量的表达形式
  
  - `let data:i8 = 5`
  
  - `let data = 5i8`

- 字符串特殊字符
  
  - `\n`、`\t`

- 变量方法，包含在变量内部的函数方法，可以对变量的值做出改变，并返回改变后的值

- 格式描述符，自定义值的表达形式并返回，不改变值
  
  - `{<浮点数变量>:.2}`表示打印小数点后两位

- 类型强制转换，`<变量名> as <变量类型>`

- 算术表达式，加、减、乘、除、取余等等

- 简化算术表达，`-= ...`
  
  - **不存在**尾缀表达式，例如：`a++;`

- 布尔类型，类型表达：`let a:bool = true;`
  
  - 反转类型表达，`!a => false`
  
  - 其他类型的表达，得到布尔类型值
    
    示例：`b.is_positive(),b.is_positive() // b为i8`
  
  - 布尔表达式，`b > 3, b <= 20`等等

- 逻辑表达式
  
  - 前置，比较的数据的数据类型必须一致，否则就进行数据类型强制转换
  
  - 等号，`==, !=`
  
  - 大于小于号，`>, <, >=, <=`
  
  - 逻辑符号
    
    - 且，`&&`
    - 或，`||`

- 字符编码
  
  - 全世界通用的一种编码格式，占用4个字节
  - 字符变量，含有内部判断方法

- 数据类型分类
  
  - 单一类型，只能包含一个值，例如整型、浮点型数据等等
  
  - 复合类型，可以包含多个值，例如数组、字符串数据等等
    
    - 数组，长度是固定的，且必须给予，只能修改数据值

- `trait`特征
  
  - 打印数组信息，`println!("{:?}", array)`

- `debug`函数
  
  - 这是一种更好地打印值、表达式的方式，甚至用于函数都可以【仅个人猜测】
  
  - 使用方式，`dbg!(2+3)`，得到结果：`2 + 3 = 5`

- 元组
  
  - 表达形式：`let employee = ("Bob", 32, "programming");`
  
  - 支持不同的数据类型同时存放

- 迭代和迭代范围
  
  - 迭代表示：`let month_days = 1..31; // 表示1-30的范围`

- 泛型
  
  - 这个数据可以是允许类型中的任意一个，以抽象方式进行定义
  
  - 当定义某个变量的时候，需要确定它的数据类型
  
  - 示例：`let data39: std::ops::Range<i16> = 1..31;`

##### 代码区

```rust
use std::any::Any;

#[allow(unused_variables)]
fn main() {
    let data1: i8 = 127;
    let data2: u8 = 255;

    let data3: i16 = -30000;
    let data4: i16 = 30000;

    let data5 = 25i8;
    let data6 = 25u8;

    let data7 = 25_32_1; // 下划线表示，最终的值会消除下划线
    println!("{data7}");

    // 根据计算机操作系统的位，来分配变量的空间大小，方便跨系统使用变量
    let data8: isize = 25;
    let data9: usize = 25;

    // 字符串及特殊字符的表示方法
    let data10: &str = "C:\\MyDocument\\new\\video.mp4";
    let data11: &str = "This is my apple,\nnot yours.";
    let data12: &str = "\tHello! Is there somebody here?";
    println!("{data10}");
    println!("{data11}");
    println!("{data12}");

    // 变量的内部函数方法
    let data13: i16 = -24;
    println!("{}", data13.abs());
    println!("{}", data13.pow(2));
    let data14: &str = "  dqwdxxx text ";
    println!("{}", data14.trim());

    // 浮点数，位数是对精度的控制
    let data15: f32 = 7.141592653589793;
    let data16: f64 = 7.141592653589793;
    let data17: f32 = 10.56;
    let data18: f32 = 10.46;
    println!("{}", data15);
    println!("{}", data16);
    println!("{}", data17.floor()); // 向下取整
    println!("{}", data17.ceil()); // 向上取整
    println!("{}", data17.round()); // 四舍五入
    println!("{}", data18.round()); // 四舍五入

    // 自定义描述符
    let data19: f32 = 10.46252;
    println!("{data19:.2}");

    // 数据类型强制转换
    let data20: i8 = 12;
    let data21: i32 = data20 as i32;
    let data22: f32 = 24.12;
    let data23: i8 = data22 as i8;
    println!("{data21} {:?}", data21.type_id());
    println!("{data23} {:?}", data23.type_id());

    // 算术表达式
    let add: f32 = 2.1 + 3.2;
    let sub: f32 = 2.1 - 3.2;
    let mul: f32 = 2.1 * 3.2;
    let div: f32 = 2.1 / 3.2;
    let mol: f32 = 9.1 % 3.2;
    println!("add: {add:.2}");
    println!("sub: {sub:.2}");
    println!("mul: {mul:.2}");
    println!("div: {div:.2}");
    println!("mol: {mol:.2}");

    // 赋值表达
    let mut data24: i8 = 8;
    println!("Now, I have {} apples.", data24);
    data24 += 1;
    println!("Add 1, I have {} apples.", data24);
    data24 -= 1;
    println!("Sub 1, I have {} apples.", data24);
    data24 *= 2;
    println!("Mul 2, I have {} apples.", data24);
    data24 /= 2;
    println!("Div 2, I have {} apples.", data24);

    // 布尔类型
    let is_beautiful: bool = true;
    println!("I have a {} face.", is_beautiful);
    println!("I have a {} face.", !is_beautiful);

    let age: i8 = 32;
    let is_young: bool = age > 18;
    println!("{}", is_young);
    println!("{} {}", age.is_negative(), age.is_positive());
    println!("I'm {} years old, can I go to a cinema? {}", age, is_young);

    // 逻辑表达式
    println!("\"Coke\" == \"Coke\", result is: {}", "Coke" == "Coke");
    println!("\"Coke\" == \"coke\", result is: {}", "Coke" == "coke");
    println!("\"Coke\" != \"Coke\", result is: {}", "Coke" != "Coke");
    println!("\"Coke\" != \"coke\", result is: {}", "Coke" != "coke");

    let data26: bool = true;
    let data27: bool = true;
    let data28: bool = false;
    let data29: bool = data26 && data27;
    let data30: bool = data26 && data27 && data28;
    println!("data29:bool = data26 && data27, result is: {}", data29);
    println!(
        "data30:bool = data26 && data27 && data28, result is: {}",
        data30
    );

    let data31: bool = true;
    let data32: bool = false;
    let data33: bool = false;
    let data34: bool = data31 || data32;
    let data35: bool = data31 || data32 || data33;
    let data36: bool = data31 || data32 || data33 && data34;
    println!("data34:bool = data31 || data32, result is: {}", data34);
    println!(
        "data35:bool = data31 || data32 || data33, result is: {}",
        data35
    );
    println!(
        "data36:bool = data31 || data32 || data33 && data34, result is: {}",
        data36
    );

    // 字符格式
    let data37: char = 'B';
    let data38: char = '🍔';
    println!(
        "char is: {}, Is a alphabetic(): {}, is_uppercase(): {}, is_lowercase(): {}",
        data37,
        data37.is_alphabetic(),
        data37.is_uppercase(),
        data37.is_lowercase()
    );
    println!("char is: {}", data38);

    // 数组
    let numbers: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // 数组是固定大小的元素，不能多也不能少
    let mut apples: [&str; 4] = ["asd", "dq12", "ffgr", "dewq441"];
    println!("apples.len: {}", apples.len());
    let empty_array: [f32; 0] = [];

    let index_1: &str = apples[0];
    let index_2: &str = apples[1];
    let index_3: &str = apples[2];
    let index_4: &str = apples[3];
    println!("index_1: {}", index_1);
    println!("index_2: {}", index_2);
    println!("index_3: {}", index_3);
    println!("index_4: {}", index_4);

    apples[0] = "ds453453";

    println!("apples[0]: {}", apples[0]);
    println!("apples[1]: {}", apples[1]);
    println!("apples[2]: {}", apples[2]);
    println!("apples[3]: {}", apples[3]);

    // trait
    let seasons: [&str; 6] = ["summer", "fall", "winter", "spring", "autumn", "spring"];
    // println!("{}", seasons);    // 这是无法直接打印的
    println!("seasons: {:?}", seasons); // 方式一
    println!("seasons: {seasons:?}"); // 方式二
    println!("seasons: {:#?}", seasons); // 漂亮格式打印数组

    // debug
    dbg!(2 + 3);
    dbg!(seasons);

    // 元组
    let employee = ("Bob", 32, "programming");
    let name = employee.0;
    let age = employee.1;
    let career = employee.2;
    let (name, age, career) = employee; // 获取元组值的另外一种方式

    dbg!(employee);
    dbg!(name);
    dbg!(age);
    dbg!(career);

    // 范围和范围迭代
    let month_days = 1..31; // 表示1-30的范围
    let month_days_two = 1..=31; // 表示1-31的范围
    let letters = 'a'..='x';

    println!("{:?}", month_days);
    println!("{:?}", month_days_two);

    /*
        dbg 被用于一个值后，这个值就不能另作他用，
        如果改变这种情况，用只能通过给dbg引用值的地址
        即：dbg!(&data)
    */
    // dbg!(month_days);
    // dbg!(month_days_two);

    for month_day in month_days {
        print!("{} ", month_day);
    }
    println!();

    for letter in letters {
        print!("{} ", letter);
    }
    println!();

    for season in seasons {
        print!("{} ", season);
    }
    println!();

    // 泛型
    let data39: std::ops::Range<i16> = 1..31;
    let data40: std::ops::Range<char> = 'a'..'g';

    // test
    let data_t01 = 13_37;
    let data_t02 = data_t01 as i16;

    let data_t03 = 12.3325;
    println!("{:.3}",data_t03);

    let with_milk = true;
    let with_sugar = false;
    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let data_t04:[i8;5] = [1,2,3,4,5];
    dbg!(data_t04);

    let data_t05 = (12,32.1,false,[2,3,4,5]);
    dbg!(data_t05);
}
```

---

#### 函数

- 定义，可以重复使用的代码块，只需要实现一次，就可以随处调用执行，并且可以执行多次。

- 名称定义，英文字母小写，且用下划线隔开

- 函数参数，函数期待传入的数据名称，格式：`名称:类型`

- 返回值
  
  - 标识：`fn name() -> [结果的表达式类型] {函数体}`
  
  - 显式返回值，会返回`return 表达式;`
  
  - 隐式返回值，会返回最后一句话的表达式结果
  
  - 元组是一种返回值类型
  
  - <u>作用域可以返回值</u>

##### 代码区

```rust
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
```

---

#### 控制流

- If语句
  
  - 表达结构：`if 条件 {代码块}`
  
  - 具有关联性，通过`else-if,else`进行关联
  
  - 具备独立性，每一个`if`语句都是一个独立的关联块
  
  - 简短表达式：`let result = if 条件 {变量值} else {变量值}`

- match语句
  
  - 代码块模式，每个语句在`{}`中
  
  - 单行模式，每个结果是一个语句，每个语句之间用逗号隔开
  
  - 含：布尔类型判断、自定义值判断，最终用`_`收尾

- iterate迭代器
  
  - `loop {}`，通过`break`跳出循环，通过`continue`开启下一次循环

- while迭代器
  
  - `while 条件 {}`，通过`continue`开启下一次循环

- 递归
  
  - `base case`，基本条件，这是指令递归停止的条件
  
  - 不需要使用`return`进行递归，直接调用函数本身即可
  
  - 连接`if`语句进行递归控制

- 调试

##### 代码区

```rust
fn main() {
    // if 条件
    let data1: bool = true;

    if true {
        println!("Hello, world!");
    }

    if false {
        println!("can't execute.");
    }

    if data1 {
        println!("data1 can execute.");
    }

    // if else 条件
    let season = "summer";

    if season == "summer" {
        println!("Now is summer day.");
    } else if season == "winter" {
        println!("Now is winter day.");
    } else if season == "spring" {
        println!("Now is spring day.");
    } else if season == "autumn" {
        println!("Now is autumn day.");
    } else {
        println!("We don't know the season that we face.");
    }

    // if 简短式
    even_or_odd(12);
    even_or_odd(17);

    // match
    let evaluation = true;

    match evaluation {
        true => {
            println!("The evaluation is true.");
        }
        false => {
            println!("The evaluation is false.");
        }
    }

    let data2 = true;
    let data3 = match data2 {
        true => 20,
        false => 40,
    };

    println!("The data2 is: {}", data2);
    println!("The data3 is: {}", data3);

    // match for season， 两种表达方式：语句块模式、单行模式
    match season {
        "summer" => {
            println!("Now is summer day.");
        }
        "winter" => {
            println!("Now is winter day.");
        }
        "spring" => {
            println!("Now is spring day.");
        }
        "autumn" => {
            println!("Now is autumn day.");
        }
        _ => {
            println!("We don't know the season that we face.");
        }
    }

    match season {
        "summer" => println!("Now is summer day."),
        "winter" => println!("Now is winter day."),
        "spring" => println!("Now is spring day."),
        "autumn" => println!("Now is autumn day."),
        _ => println!("We don't know the season that we face."),
    }

    // match 匹配多个值
    let number = 2;
    match number {
        1 | 3 | 5 | 7 | 9 | 11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 | 27 | 29 => {
            println!("The number is odd.")
        }
        2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 | 26 | 28 => {
            println!("The number is odd.")
        }
        _ => println!("The number is unknow."),
    }

    // 本初有两种做法
    let value = 8;
    match value {
        value if value % 2 == 0 => println!("The value is even"),
        _ => println!("The number is even"),
    }

    match value {
        value if value % 2 == 0 => println!("The value is even"),
        x if x % 2 != 0 => println!("The number is even"),
        _ => println!("The value is unknow."),
    }

    match value {
        value if value % 2 == 0 => println!("The value is even"),
        x if x % 2 != 0 => println!("The number is even"),
        _ => unreachable!(),
    }

    // loop 迭代循环
    let mut value = 10;

    loop {
        if value == 0 {
            break;
        }
        println!("The number is: {}", value);
        value -= 1;
    }
    println!("Now, The number is: {}", value);

    // continue 语句 与 loop 循环结合

    let mut seconds = 23;

    loop {
        if seconds <= 0 {
            println!("Time is over.");
            break;
        }

        if seconds % 2 == 0 {
            println!("The seconds is even. Jump. {}s", seconds);
            seconds -= 3;
            continue;
        }

        println!("Time is running... {}s", seconds);
        seconds -= 1;
    }

    seconds = 23;
    println!("while version.");
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("The seconds is even. Jump. {}s", seconds);
            seconds -= 3;
            continue;
        }

        println!("Time is running... {}s", seconds);
        seconds -= 1;
    }

    // 递归
    countdown(12);

    // test
    println!("color is: {}", color_to_number("blue"));
    println!("color is: {}", factorial(5));
    println!("color is: {}", factorial_recursion(5));
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is: {}", result);
}

fn countdown(seconds: i16) {
    if seconds <= 0 {
        println!("finished seconds.");
    } else {
        println!("{} seconds...", seconds);
        countdown(seconds - 1)
    }
}
/*
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
*/
fn color_to_number(color: &str) -> i16 {
    /*if color == "red"{
        return 1;
    }else if color == "green" {
        return 2;
    }else if color == "blue" {
        return 3;
    }else {
        return 0;
    }*/

    /*
        return if color == "red" {
            1
        } else if color == "green" {
            2
        } else if color == "blue" {
            3
        } else {
            0
        }
    */

    match color {
        "red" => {
            return 1;
        }
        "green" => {
            return 2;
        }
        "blue" => {
            return 3;
        }
        _ => {
            return 0;
        }
    }

    /*match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }*/

    /*return match color {
        "red" => {
            1
        }
        "green" => {
            2
        }
        "blue" => {
            3
        }
        _ => {
            0
        }
    }*/
}

fn factorial(number: i32) -> i32 {
    let mut sum = 1;
    let mut count = number;

    while count >= 1 {
        sum *= count;
        count -= 1;
    }

    // 等同于 return sum;
    sum
}

/*
    只要返回了递归函数，那么这个值就会被一起带到下一层
*/
fn factorial_recursion(number: i32) -> i32 {
    if number < 1 {
        return 1;
    } else {
        // 这句话等同于 return number * factorial_recursion(number - 1)
        number * factorial_recursion(number - 1)
    }
}
```

---

#### 所有权

- 定义，所有权是编译器检查的一系列规则，用于确保内存不会出错

- 内存，是程序运行时为动态数据而建立的存储地

- 不再被使用的时候，需要被释放空间

- 编程语言接入了不同的存储管理策略

- 垃圾处理机制，自动回收不使用的数据，导致内存被不必要的占用空间，同时降低程序运行的速度

- `Rust`所有权，用于管理计算机内存，它是一组规则
  
  - 如果违反权限规定，那么无法通过编译器
  
  - 运行速度更快，更不容易出现错误
  
  - 每一个数值都是一个权限主，负责清理不使用的数据

- 堆和栈，这是两种不同的计算机存储领域，用于读取数据和写入数据
  
  - 栈存储，是一系列连续的存储空间，后进先出，出栈入栈
    
    - 都有一个固定的大小空间
  
  - 堆存储，是一个巨大的存储空间，负责存储大小未知的数据
    
    - 产生动态空间，并通过内存分配器为数据分配空间
    
    - 内存分配器，会为数据提供一个引用，即地址，指向存储空间。因此变量可以通过存储引用，再通过引用来找到所有的数据。【会花费大量时间寻找足够的内存空间】

- 总结
  
  - 所有权的目的是释放存储空间
  
  - 所有权减少重复的堆数据，并且清理不再使用的数据

- `String`类型
  
  - 既不使用栈存储也不使用堆存储，而是直接编译成二进制文件
  
  - 声明方式：`let str:&str = "testest";`
  
  - 声明方式二：`let text = String::new()`
  
  - 栈中保存的字符串数据：{引用指针、字符串长度、容量大小}【容量指的是堆中的容量大小】
  
  - `push_str`：新字符串推送给字符串变量后，该字符串的长度、容量会发生相应的变化
  
  - `move`操作：将所有权从一个变量转移给另一个变量
  
  - `drop`操作：释放字符串变量的堆空间和栈空间

- 借用所有权
  
  - 直接赋值另外一个变量，那么原来值的所有权会被转移，但是可变性权益不会被转移，例如：
    
    ```rust
    let drink = String::from("Snapple");
    let beverage = drink;
    ```
  
  - 在没有获得所有权的情况下，操作该变量
  
  - 借用运算符【地址引用运算符】：`&`，这是用于获取数据值的引用地址，不会导致所有权的移动
  
  - 引用是一种指针类型
  
  - 解引用运算符：`*`，这是用于获取引用地址的数据值

- 字符引用【`String`,`&String`,`str`,`&str`】
  
  - `String`：创建一个字符串对象，**拥有字符串方法，会传递所有权**
  - `&String`：创建一个字符串对象的引用
  - `str`：创建一个字符串对象的引用，如`let a = "asdasd"`
  - `&str`：创建一个字符串对象的引用的引用

- 字符所有权的变化

##### 代码区

```rust
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
```

---

#### 引用和所有权转移

- 向函数中传递字符串对象的可修改引用
- 对象引用可以共存，对象引用且修改完毕之后，才能被再次借用
- 悬空引用（空指针）
- 对数组和元组进行引用

特别地，这两种方式可以对字符串对象进行引用

1. 设置可修改的字符串对象

2. 设置对字符串对象的可修改引用

代码如下：

```rust
fn make_empty(mut content: String) {
    content.clear()
}
 
fn make_empty(content: &mut String) {
    content.clear()
}
```

##### 代码区

```rust
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
```

---

#### 切片

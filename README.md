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
  
  - cargo run --bin <文件名称||项目名称>，指定生成可调式版本代码，并运行
  
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

**代码区**

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

**代码区**

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

**代码区**

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

**代码区**

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

**代码区**

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
- 一个字符串对象可以拥有多个不可变引用变量和同时只能拥有一个可变引用变量

代码如下：

```rust
fn make_empty(mut content: String) {
    content.clear()
}

fn make_empty(content: &mut String) {
    content.clear()
}
```

**代码区**

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

- 借用一个字符串对象的切片

- 块中创建字符串引用，并返回部分字符串切片

- 字符串切片的长度

- 字符串语法快捷实现

- 字符串切片与函数参数

- 切片数组

- 数组引用与数组，数组引用的函数定义

- 注意点
  
  - 对于`city`来说，`let city = String::from("Dallas");`
  
  - `&city`的类型是`&String`，即字符串对象引用
  
  - `&city[..]`的类型是`&str`，即字符串引用

**代码区**

```rust
fn main() {
    // 1. 借用一个字符串对象的切片
    /*
        他们均属于字符串切片，只是范围不同
        action_hero // 第二种创建方式
        first_name
        last_name
    */
    // let action_hero = String::from("Hero BobSunBoy");   // 字符串对象，直接提供字符串
    let action_hero = "Hero BobSunBoy"; // 字符串对象，只提供一个字符串的地址
    // let string_reference = &action_hero[0..4];  // Hero
    // let string_reference = &action_hero[..4];   // Hero
    let string_reference = &action_hero[5..]; // BobSunBoy
    println!("{}", string_reference);

    let first_name = &action_hero[0..4];
    let last_name = &action_hero[5..];
    println!("So you name is: {} {}", first_name, last_name);

    // 2. 块中创建字符串引用，并返回部分字符串切片
    let final_name = {
        // let action_hero = String::from("Hero BobSunBoy");   // 只创建字符串对象，取不到字符串内容
        let action_hero = "Hero BobSunBoy"; // 连同地址引用和部分堆空间，一起分配给变量 final_name
        &action_hero[0..4]
    };

    println!("{}", final_name);

    // 3. 字符串切片的长度
    let food = "pizza🍕"; // 图标字符占四个字符长度
    println!("Food length: {}", food.len());
    let pizza_slice = &food[0..3];
    println!("pizza_slice length: {}", pizza_slice.len());

    // 4. 字符串语法快捷实现
    let action_origin = String::from("Hero BobSunBoy");

    let first_origin = &action_origin[..5];
    println!("His first name is: {}", first_origin);

    let last_origin = &action_origin[5..];
    println!("His last name is: {}", last_origin);

    let full_origin = &action_origin;
    println!("His full name is: {}", full_origin);

    // 5. 字符串切片与函数参数
    let action_double = String::from("Hero BobSunBoy");
    do_hero_stuff(&action_double);
    let action_double_another = "Hero BobSunBoy";
    do_hero_stuff(action_double_another);

    // 6. 切片数组
    let values = [49, 82, 63, 50, 33, 21];

    let slice_values = &values[1..3];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values[3..];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values[..3];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values[..];
    println!("slice_values: {slice_values:?}");

    let slice_values = &values;
    println!("slice_values: {slice_values:?}");

    // 7. 数组引用与数组，数组引用的函数定义
    let values = [49, 82, 63, 50, 33, 21];

    let values_reference = &values;
    print_length(values_reference);

    let three_reference = &values[..3];
    print_length(three_reference);

    // 8. 可变的数组切片
    let mut my_array = [12, 3, 2, 41, 66, 7];
    let my_slice = &mut my_array[2..4]; // 数组的可变引用

    println!("My slice is: {:?}", my_slice);
    my_slice[0] = 48;

    println!("My slice is: {:?}", my_slice);
    println!("My array is: {:?}", my_array); // 引用修改数据后，堆空间的值会发生变化

    // 9. test
    let mut cereals = [
        "Cookie Crisp",
        "Cinnamon Toast Crunch",
        "Frosted Flakes",
        "Cocoa Puffs",
        "Captain Crunch",
    ];

    let first_two = &cereals[0..2];
    println!("first_two: {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("mid_three: {:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("last_three: {:?}", last_three);

    last_three[2] = "Lucky Charms";
    println!("cereals: {:?}", cereals);

    let cookie_crisp = String::from("Cookie Crisp");

    let cookie = &cookie_crisp[0..7];
    println!("cookie: {}", cookie);

    let cocoa_puffs = "Cocoa Puffs";

    let puffs = &cocoa_puffs[6..];
    println!("puffs: {}", puffs);
}

fn do_hero_stuff(hero: &str) {
    // hero: &String 仅支持字符串对象引用
    // hero: &str 支持字符串对象引用、字符引用，范围支持上更好
    // 可以将 &String -> &str ，反过来无法实现
    println!("{} saves the day", hero);
}

fn print_length(reference: &[i32]) {
    // 数组或数组引用的描述：reference:[i32;6],reference:&[i32;6]
    // 数组引用的类型描述会更加松解：reference:&[i32]
    println!("the length is: {}", reference.len());
}
```

---

#### 数据结构

- 数据结构定义

- 创建数据结构实例

- 读取数据结构的属性

- 复写数据结构的领域

- 创建一个返回数据结构的函数

- 数据结构简洁实例化方法

- 数据结构更新语法

- 传递数据结构给函数

- 数据结构的debug

- 定义数据结构方法

- 返回所有权设计

- 函数引入多个参数

- 函数中调用函数

- 关联函数, 构造函数设计

- 多模块接口

- 构建模式

- 元组数据结构

补充

1. 数据结构的命名使用驼峰大小写

2. 注意区分数据结构的内部函数和内联函数

3. 使用打印方法`println!("{:#?}",xxxStruct)`时，注意要在数据结构名称上注释`#[derive(Debug)`

4. 使用双引号来定义剩余不变的数据结构属性`{name:'xx',..YStruct}`

5. 声明数据结构方法函数的关键字`impl`

6. **使用实例对象，会导致所有权转移，所以一般就使用实例对象引用**

**代码区**

```rust
// 9. 数据结构的debug - part1
#[derive(Debug)] // 允许打印 数据结构 的规范表达

// 1. 数据结构定义
struct Coffee {
    // 命名规则：各单词首字母大写
    name: String,
    price: f64,
    is_available: bool,
}

// 10. 定义数据结构方法 - part1
#[derive(Debug)]
struct RoseSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// 16. 构建模式 - part1
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl RoseSong {
    // 14. 关联函数, 构造函数设计
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        // 返回值可以表达为：RoseSong, Self
        // 如果不包含self，那么rust 会认为这是一个关联函数
        /*RoseSong {
            title,
            release_year,
            duration_secs,
        }*/

        Self {
            title,
            release_year,
            duration_secs,
        }
    }

    // 声明数据结构方法
    /*fn display_song_info(self: Self) {
        // 传入方法一： self:RoseSong
        // 传入方法二： self:Self: Self 表示 RoseSong，本处的数据结构类型
        // 传入方法三： self
        // 对数据结构体的数据引用
        // part1 - 不可变数据结构值
        println!("-- RoseSong --");
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_secs: {}", self.duration_secs);
    }*/

    /*fn double_length(mut self) {
        self.duration_secs *= 2;
        println!("Total Information: {:#?}", self);
        // self.display_song_info();
    }*/
    // 11. 返回所有权设计
    fn display_song_info(self: &Self) {
        // 传入方法一： self: &RoseSong
        // 传入方法二： self: &Self: &Self 表示 RoseSong 的引用，引用本处的数据结构类型
        // 传入方法三： &self
        // 推荐写二、三方法，有利于数据结构名称维护，而不至于修改所有函数代码
        // 对数据结构体的数据引用
        // part1 - 不可变数据结构值
        println!("-- RoseSong --");
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_secs: {}", self.duration_secs);
        println!("Year since Release: {}", self.years_since_release());
    }

    fn double_length(self: &mut Self) {
        // 对实例的可变引用
        // 传入方法一： self:&mut RoseSong
        // 传入方法二： self:&mut Self: &Self
        // 传入方法三： &mut self
        self.duration_secs *= 2;
    }

    // 12. 函数引入多个参数
    fn is_longer_that(self: &Self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    // 13. 函数中调用函数
    fn years_since_release(self: &Self) -> u32 {
        2029 - self.release_year
    }
}

// 15. 多模块接口
impl RoseSong {
    // 可以存在多个相关于一个数据结构的接口模块
    fn display_song_info_two(self: &Self) {
        println!("-- RoseSong Version 2.0 --");
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_secs: {}", self.duration_secs);
        println!("Year since Release: {}", self.years_since_release());
    }
}

// 16. 构建模式 - part2
impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(self: &mut Self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(self: &mut Self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(self: &mut Self, new_hard_drive_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive_capacity;
        self
    }

    fn print_computer_info(self: &Self) {
        println!("-- Computer --");
        println!("cpu: {}", self.cpu);
        println!("memory: {}", self.memory);
        println!("hard_drive_capacity: {}", self.hard_drive_capacity);
    }
}

// 17. 元组数据结构 - par1
struct ShortDuration(u32, u32);
struct LongDuration(u32, u32);

// 18. 单元数据结构 - part1
struct Empty; // 这是一个单元数据结构

// 19. test - part1
#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(self: &mut Self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(self: &mut Self) -> &mut Self {
        self.price *= 1.2;
        self
    }

    fn itinerary(self: &mut Self) -> &mut Self {
        println!("-- Flight --");
        println!(
            "origin: {} -> destination: {}",
            self.origin, self.destination
        );
        println!("price: {}", self.price);
        println!("passengers: {}", self.passengers);
        self
    }
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

    // 9. 数据结构的debug
    // #[allow(unused_variables)] 单行指令 - 允许存在未使用的变量
    let mocha_debug = make_coffee(String::from("Debug Coffee"), 21.19, false);

    println!("{:?}", mocha_debug);
    println!("{:#?}", mocha_debug);

    // 10. 定义数据结构方法
    let mut rose_song_one = RoseSong {
        title: String::from("Gone."),
        release_year: 2021,
        duration_secs: 207,
    };

    // rose_song_one.display_song_info(); // rose_song_one 的所有权转移到了这里

    // rose_song_one.double_length();

    // 11. 返回所有权设计
    rose_song_one.display_song_info();
    rose_song_one.double_length();

    // 12. 函数引入多个参数
    let rose_song_two = RoseSong {
        title: String::from("Two box Seconds."),
        release_year: 2026,
        duration_secs: 128,
    };

    if rose_song_one.is_longer_that(&rose_song_two) {
        println!("The compare result is: rose_song_one win !!!");
    } else {
        println!("The compare result is: rose_song_two win !!!");
    }

    // 13. 函数中调用函数
    rose_song_two.display_song_info();

    // 14. 关联函数, 构造函数设计
    /*
        关联函数
        内置函数，可以引用对象本身
    */
    let rose_song_three = RoseSong::new(String::from("Three Candy."), 2027, 256);
    rose_song_three.display_song_info();

    // 15. 多模块接口
    rose_song_three.display_song_info_two();

    // 16. 构建模式
    let mut computer = Computer {
        cpu: String::from("Intel"),
        memory: 16,
        hard_drive_capacity: 1024,
    };

    computer.print_computer_info();

    // 实现方法一
    /*computer.upgrade_cpu(String::from("Amd"));
    computer.upgrade_memory(32);
    computer.upgrade_hard_drive_capacity(4096);*/

    // 实现方法二
    computer
        .upgrade_cpu(String::from("Amd"))
        .upgrade_memory(32)
        .upgrade_hard_drive_capacity(4096);

    computer.print_computer_info();

    // 17. 元组数据结构
    /*
        从名称上区分数据，从而避免原始元组数据类型相同的问题
        以及，函数参数类型命名上的问题
        这个问题已经得到官方的修复和错误捕捉
    */
    let work_shift = ShortDuration(8, 0);
    println!("{} hour {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} year {} month", era.0, era.1);

    // let era_raw = (5,3);
    // go_to_work(era_raw);
    go_to_work(era);

    // 18. 单元数据结构
    let empty = (); // 这是一个单元结构体
    let empty_struct = Empty; // 这是一个单元结构体

    // 19. test
    let mut flight_one = Flight::new(
        String::from("BeiJing"),
        String::from("ShangHai"),
        1284.231,
        54,
    );

    println!("Origin Info: {:#?}", flight_one);

    flight_one
        .change_destination(String::from("HongKong"))
        .increase_price()
        .itinerary();

    let mut flight_two = Flight {
        origin: String::from("ChongQing"),
        destination: String::from("GuangZhou"),
        ..flight_one
    };

    println!("Origin Info: {:#?}", flight_two);
    flight_two.itinerary();
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

fn go_to_work(value: LongDuration) {
    println!("Test question: {} hours {} minutes", value.0, value.1);
}
```

---

#### 枚举

- 介绍枚举

- 枚举关联值 part1

- 枚举关联值 part2

- 枚举存储的简单描述

- 结构变量

- 枚举中的枚举

- 匹配关键字 part1

- 匹配关键字 part2

- 匹配关键字 part3

- 在枚举中定义方法

- 匹配关键字 part4

- 匹配关键字 part5

- if 结构运用

- if 与 else 结构运用

补充

1. 枚举中定义值，可以自己声明名称，值的类型可以不声明，支持的类型如下：
   
   - 标准的数据类型，整型、字符型数据
   
   - 枚举类型
   
   - 数据结构类型
   
   也许还有很多可以接入的类型

2. 枚举也如数据结构一样，支持枚举方法的设计，有两种枚举方法：
   
   - 枚举内部函数，支持方法链的使用方式
   
   - 枚举一般函数，即不调用自身的函数

3. `if let`结构，匹配对应的枚举值是否一致，一致是一条支线、不一致也是一条支线，同时也支持使用对应枚举变量的内部数据值

**代码区**

```rust
// 1. 介绍枚举 - part1
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

// 2. 枚举关联值 - part1
#[derive(Debug)]
enum PaymentMethodType {
    // CreditCard(String, i32, bool),
    CreditCard(String),
    DebitCard(String),
    // PayPal(String, String),
    // PayPal { name: String, password: String },
    PayPal(Credentials),
    Cash, // 不存储任何数据
}
struct Card {
    rank: String,
    suit: CardSuit,
}

// 5. 结构变量 - part1
#[derive(Debug)]
struct Credentials {
    name: String,
    password: String,
}

// 6. 枚举中的枚举 - part1
#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

// 7. 匹配关键字 - part1
enum OperationSystem {
    Windows,
    MacOs,
    Linux,
}

// 9. 匹配关键字 - part3
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

// 10. 在枚举中定义方法 - part1
impl LaundryCycle {
    fn wash_laundry(self: &Self) {
        match self {
            LaundryCycle::Cold => println!("LaundryCycle is Cold"),
            LaundryCycle::Hot { temperature } => {
                println!(
                    "LaundryCycle is temperature, its value is : {}",
                    temperature
                );
            }
            LaundryCycle::Delicate(fabric) => {
                println!("LaundryCycle is String, its value is : {}", fabric);
            }
        }
    }
}

// 11. 匹配关键字 - part4
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(self: &Self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your Item has been delivered for Ordered or Packed")
            }
            other_status => {
                println!("Your Item has been delivered for Shipped or Delivered, {other_status:#?}")
            }
        }
    }
}

// 11. 匹配关键字 - part5
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self: &Self) {
        match self {
            Milk::Lowfat(2) => {
                println!("The milk is lowfat with 2.")
            }
            Milk::Lowfat(percent) => {
                println!("The milk is lowfat with {}.", percent)
            }
            Milk::Whole => {
                println!("The milk is whole fat.");
            }
            other_milk => {
                println!("The milk is other_milk.");
            }
        }
    }
}

// test
#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}
#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(self: &Self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, months) => println!(
                "You have limited access to the site's premium features for {price} for {months} months"
            ),
            Subscription::Premium { tier } => println!(
                "You have full access to the site's premium features. Your tier is {tier:?}"
            ),
        }
    }
}

fn main() {
    // 1. 介绍枚举
    let first_card = CardSuit::Diamonds;
    let mut second_card = CardSuit::Hearts;
    second_card = CardSuit::Clubs;
    println!("{:#?}", first_card);
    println!("{:#?}", second_card);

    let card_suits = [
        CardSuit::Hearts,
        CardSuit::Diamonds,
        CardSuit::Clubs,
        CardSuit::Spades,
    ];

    let card_suits_two = (CardSuit::Hearts, CardSuit::Spades);

    println!("{:?}", card_suits);
    println!("{:?}", card_suits_two);

    // 2. 枚举关联值 - part1
    let visa = PaymentMethodType::CreditCard(String::from("0034-2331-221"));
    let mastercard = PaymentMethodType::DebitCard(String::from("0998-4412-133"));

    println!("{:#?}", visa);
    println!("{:#?}", mastercard);

    // 3. 枚举关联值 - part2
    /*let mut my_visa = PaymentMethodType::CreditCard(String::from("0034-2331-221"));
    my_visa = PaymentMethodType::PayPal(String::from("3877-213"), String::from("1221-399"));

    println!("{:#?}", my_visa);*/

    // 4. 枚举存储的简单描述
    // 5. 结构变量

    let paypal_credentials = Credentials {
        name: String::from("wiiw@qq.com"),
        password: String::from("228371"),
    };

    /*let my_visa = PaymentMethodType::PayPal{
        name: String::from("wiiw@qq.com"),
        password: String::from("228371"),
    };*/

    let my_visa = PaymentMethodType::PayPal(paypal_credentials);

    println!("my_visa: {:#?}", my_visa);

    // 6. 枚举中的枚举
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let meal = RestaurantItem::VeganPlate;

    println!("lunch: {:#?}", lunch);
    println!("dinner: {:#?}", dinner);
    println!("meal: {:#?}", meal);

    // 7. 匹配关键字 - part1
    let number = 5;

    match number {
        5 => println!("The number is 5"),
        8 => println!("The number is 8"),
        _ => println!("The number is not 5 or 8"),
    }

    let computer_1 = OperationSystem::Linux;
    let computer_2 = OperationSystem::Windows;
    let computer_3 = OperationSystem::MacOs;

    println!("{}", years_since_release(computer_1));
    println!("{}", years_since_release(computer_2));
    println!("{}", years_since_release(computer_3));

    // 8. 匹配关键字 - part2
    // 扩展匹配模块，在 match 模块中实现打印和返回值

    // 9. 匹配关键字 - part3
    let type_1 = LaundryCycle::Cold;
    let type_2 = LaundryCycle::Hot { temperature: 31 };
    let type_3 = LaundryCycle::Delicate(String::from("Beautiful!!"));

    wash_laundry(&type_1);
    wash_laundry(&type_2);
    wash_laundry(&type_3);

    // 10. 在枚举中定义方法
    type_1.wash_laundry();
    type_2.wash_laundry();
    type_3.wash_laundry();

    // 11. 匹配关键字 - part4
    let status_1 = OnlineOrderStatus::Ordered;
    let status_2 = OnlineOrderStatus::Shipped;

    status_1.check();
    status_2.check();

    // 11. 匹配关键字 - part5
    let milk_1 = Milk::Lowfat(2);
    let milk_2 = Milk::Lowfat(23);
    let milk_3 = Milk::Whole;

    milk_1.drink();
    milk_2.drink();
    milk_3.drink();

    // 12. if 结构运用
    /*
        条件中就是判断左边变量与右边变量是否一致
        本处没有返回值
    */
    let milk_if_1 = Milk::Whole;
    let milk_if_2 = Milk::Lowfat(12);
    let milk_if_3 = Milk::NonDairy {
        kind: String::from("Plant"),
    };

    let data_milk_1 = if let Milk::Whole = milk_if_1 {
        println!("The milk is Milk::Whole.");
    } else {
        println!("The milk is other Milk.");
    };

    let data_milk_2 = if let Milk::Whole = milk_if_2 {
        println!("The milk is Milk::Whole.");
    } else {
        println!("The milk is other Milk.");
    };

    let data_milk_3 = if let Milk::NonDairy { kind } = milk_if_3 {
        println!("The milk is Milk::NonDairy{{{}}}.", kind);
    } else {
        println!("The milk is other Milk.");
    };

    println!("data_milk_1 : {:?}", data_milk_1);
    println!("data_milk_2 : {:?}", data_milk_2);
    println!("data_milk_3 : {:?}", data_milk_3);

    // 13. if 与 else 结构运用
    let milk_if_4 = Milk::Lowfat(2);

    // 相等时 值会扩展出范围区域
    /*let Milk::Lowfat(percent) = milk_if_4 else {
        println!("The milk is wrong Milk.");
        return;
    };

    println!("{percent} % Lowfat milk");

    // 不相等时 值不会扩展出范围区域
    let Milk::NonDairy { kind } = milk_if_4 else {
        println!("The milk is wrong Milk.");
        return;
    };

    println!("{kind} % Lowfat milk");*/

    // test
    let instance_1 = Subscription::Free;
    let instance_2 = Subscription::Premium { tier: Tier::Gold};
    let instance_3 = Subscription::Basic(12.4,6);

    instance_1.summarize();
    instance_2.summarize();
    instance_3.summarize();
}

fn years_since_release(os: OperationSystem) -> u32 {
    // 会要求覆盖所有的枚举属性
    match os {
        OperationSystem::Windows => {
            println!("The computer System is windows");
            39
        }
        OperationSystem::MacOs => {
            println!("The computer System is MacOs");
            21
        }
        OperationSystem::Linux => {
            println!("The computer System is MacOs");
            7
        }
    }
}

fn wash_laundry(cycle: &LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => println!("LaundryCycle is Cold"),
        LaundryCycle::Hot { temperature } => {
            println!(
                "LaundryCycle is temperature, its value is : {}",
                temperature
            );
        }
        LaundryCycle::Delicate(fabric) => {
            println!("LaundryCycle is String, its value is : {}", fabric);
        }
    }
}
```

---

#### 泛型

- 泛型介绍
  
  - 社区一般将字母`T`称之为泛型类型

- 泛类操作符

- 多泛型

- 数据结构中的泛型

- 接入模块中的泛型

- 接入模块中的泛型 - part2

- 接入枚举中的泛型

**代码区**

```rust
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

// 6. 接入模块中的泛型 - part2
// 接入任意类型的数据
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

// 7. 接入枚举中的泛型 - part1
#[derive(Debug)]
enum Cheesesteak<T> {
    Plain,
    Topping(T),
}
// 8. test - part1
#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:#?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
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
    println!("treasure_chest_1.treasure: {}", &treasure_chest_1.treasure);
    treasure_chest_1.clean_treasure();
    println!("treasure_chest_1.treasure: {}", &treasure_chest_1.treasure);

    println_self(treasure_chest_2.amount_treasure());

    // 6. 接入模块中的泛型 - part2
    println!(
        "treasure_chest_1.captain: {}",
        &treasure_chest_1.capital_captain()
    );

    // 7. 接入枚举中的泛型
    let data_enum_1 = Cheesesteak::Topping(String::from("Mushroom"));
    let data_enum_2 = Cheesesteak::Topping("Mushroom small");
    let data_enum_3 = Cheesesteak::Topping("Mushroom small toString".to_string());
    let data_enum_4 = Cheesesteak::Topping(&data_enum_3);
    let mut data_enum_5: Cheesesteak<String> = Cheesesteak::Plain;
    data_enum_5 = Cheesesteak::Topping("sadqq1213".to_string());

    println!("{:?}", data_enum_1);
    println!("{:?}", data_enum_2);
    println!("{:?}", data_enum_3);
    println!("{:?}", data_enum_4);
    println!("{:?}", data_enum_5);

    // 8. test
    let data_1 = ChatMessage {
        content: "Mushroom",
        time: String::from("12:00"),
    };

    let data_2 = ChatMessage {
        content: "Night".to_string(),
        time: String::from("13:14"),
    };

    let data_3 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("21:33"),
    };

    data_3.consume_entertainment();

    println!("{:?}", data_1.retrieve_time());
    println!("{:?}", data_2.retrieve_time());
    println!("{:?}", data_3.retrieve_time());
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
```

---

#### 可选的结果枚举（枚举进阶）

- 可选枚举

- 可选枚举实例

- 解包和期待的方法

- 枚举匹配关键字

- 从函数中返回枚举

- 顶级可选枚举

- 解构或方法

- 源头构建 Option 对象

- 结果枚举

- 结果枚举实例

- 从函数中返回结果枚举

- 结果方法
  
  - 通过`match`结构可以取得值
  
  - 通过`wrap`系列方法也可以取得值

- 结果函数里的结构方法

- 循环结构

- 补充
  
  - `Result`里的`Ok`,`Err`属性不会将所有权转移给另一个对象，只会复制给另一个对象；`Some`枚举对象的所有权会发生转移
  
  - `Some`要求覆盖每一种可能性
  
  - `Result`要求类型声明时，覆盖正确和错误两种类型

**代码区**

```rust
// 8. 源头构建 Option 对象 - part1
#[derive(Debug, Copy, Clone)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn unwarp(self) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Have no value."),
        }
    }

    fn unwarp_or(self, fallback_value: T) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

// 15. test - part1
#[derive(Debug)]
struct Food {
    name: String,
}
#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            Some(Food {
                name: "Uni Sashimi".to_string(),
            })
        } else {
            Some(Food {
                name: "Strip Steak".to_string(),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err("Sorry, we have a mice problem.".to_string());
        }

        if address.is_empty() {
            Err("No delivery address specified".to_string())
        } else {
            Ok(Food {
                name: "Burger".to_string(),
            })
        }
    }
}

fn main() {
    // 1. 可选枚举
    let data_enum_1 = Option::Some(5);
    let data_enum_2 = Option::Some("hello");
    let data_enum_3 = Option::Some(true);
    let data_enum_4 = Option::Some("qqweea".to_string());
    let data_enum_5: Option<i16> = Option::Some(12); // 同质表达 - 1
    let data_enum_6 = Option::<i16>::Some(5); // 同质表达 - 1
    let data_enum_7: Option<&str> = Option::None; // 表达为None时，需要指明变量的类型

    // 2. 可选枚举实例
    let data_enum_8 = [
        String::from("ssa123"),
        String::from("pco312"),
        String::from("ckl321"),
    ];

    let data_enum_9: Option<&String> = data_enum_8.get(2); // 获得字符串引用
    println_self(&data_enum_9);

    let data_enum_10: Option<&String> = data_enum_8.get(12);
    println_self(&data_enum_10);

    // 3. 解包和期待的方法
    let data_enum_11 = data_enum_9.unwrap(); // 解包裹方法
    println_self(&data_enum_11);

    // let data_enum_12 = data_enum_10.unwrap();    // None 值无法解除捆绑
    // println_self(&data_enum_12);

    let data_enum_13 = data_enum_9.expect("Have no get a value."); // 期待值会进行解包处理，但在出错时打印文字描述
    println_self(&data_enum_13);

    // let data_enum_14 = data_enum_10.expect("Have no get a value."); // 期待值会进行解包处理，但在出错时打印文字描述
    // println_self(&data_enum_14);

    // 4. 枚举匹配关键字
    let data_enum_14: Option<&String> = data_enum_8.get(2);

    match data_enum_14 {
        Option::Some(instrument) => println!("Match Result: {:?}", instrument),
        Option::None => println!("Match Result: None"),
    }

    play(data_enum_8.get(1));
    play(data_enum_8.get(12));

    // 5. 从函数中返回枚举
    let availability_1 = is_item_in_stock(true, true);
    let availability_2 = is_item_in_stock(true, false);
    let availability_3 = is_item_in_stock(false, false);

    println_self(&availability_1);
    println_self(&availability_2);
    println_self(&availability_3);

    match availability_1 {
        // Some(value) => println!("Func Match Result: {:?}", value),  // 收集所有值
        Some(true) => println!("Func Match Result: true"),
        Some(false) => println!("Func Match Result: false"),
        None => println!("Func Match Result: None"),
    }

    // 6. 顶级可选枚举
    /*
        简写方式：
        - Option::Some() -> Some()
        - Option::None -> None
    */

    // 7. 解构或方法
    let availability_4 = Some(12);
    let availability_5: Option<i32> = None; // 为 None 指明数据类型

    println!("{}", availability_4.unwrap_or(0));
    println!("{}", availability_5.unwrap_or(1));

    // 8. 源头构建 Option 对象
    let availability_6 = MyOption::Some(99);
    let availability_7: MyOption<i32> = MyOption::None;

    println!("{}", availability_6.unwarp());
    // println!("{}",availability_7.unwarp()); // 因为没有数值，将引起恐慌
    println!("{}", availability_7.unwarp_or(32));

    // 9. 结果枚举
    let data_enum_15: Result<i32, &str> = Result::Ok(12);
    let data_enum_16: Result<i32, &str> = Result::Err("Something is wrong.");

    // println!("{data_enum_15:?}");
    // println!("{:?}",data_enum_15);

    println_self(&data_enum_15);
    println_self(&data_enum_16);

    // 10. 结果枚举实例
    let text = "50";
    let text_number = text.parse::<i32>();

    println!("{:?}", &text_number);

    // 11. 从函数中返回结果枚举
    // 本处我进行了解包以及错误处理
    println!("divide result is: {:?}", divide(12.0, 4.0).unwrap_or(1.0));
    println!("divide result is: {:?}", divide(12.0, 0.0).unwrap_or(0.0));

    let result_enum_1 = divide(13.4, 1.2);

    match result_enum_1 {
        Ok(val) => println!("the Result is {}", val),
        Err(message) => println!("Func Match Result: {:?}", message),
    }

    // 12. 结果方法
    // 参见 11

    // 13. 结果函数里的结构方法
    println!("{:?}", operation(true));
    println!("{:?}", operation(false));

    let my_result = operation(true);
    let content = match my_result {
        Ok(message) => message.to_string(),
        Err(error) => error.to_string(),
    }; // 获得 Result 的解构结果

    println!("{:?}", content);
    println!("{:?}", my_result);

    // 14. 循环结构
    let mut data_enum_17 = vec!["asda", "ddxz111", "cjid115"];

    while let Some(data) = data_enum_17.pop() {
        println!("the data is :{:?}", data);
    }

    // 15. test

    let restaurant_1 = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    println!("{:?}", restaurant_1.chef_special());
    println!("{:?}", restaurant_1.deliver_burger("123 Elm Street"));

    let restaurant_2 = Restaurant {
        reservations: 12,
        has_mice_infestation: false,
    };

    println!("{:?}", restaurant_2.chef_special());
    println!("{:?}", restaurant_2.deliver_burger(""));
    println!("{:?}", restaurant_2.deliver_burger("332 EPL store"));

}

fn println_self<T: std::fmt::Debug>(value: &T) {
    println!("{:?}", value);
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Func Match Result: {:?}", instrument),
        Option::None => println!("Func Match Result: None"),
    }
}

fn is_item_in_stock(item_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

// 11. 从函数中返回结果枚举 - part1
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Denominator is zero."))
    } else {
        Ok(numerator / denominator)
    }
}

// 13. 结果函数里的结构方法 - part1
// 这样的设计方法是为了避免所有权被转移
// 同时实现原对象获得结果对象，而不是结果
fn operation(success: bool) -> Result<&'static str, &'static str> {
    if success { Ok("Success") } else { Err("Error") }
}
```

---

#### 向量

- 创建向量

- 增加元素、删除元素

- 读取元素
  
  - 不可以直接读取元素，因为所有权无法直接移动

- get方法读取元素
  
  - 会返回带着值引用的`Option`对象，避免覆盖所有权

- 向量的所有权
  
  - 如果向量中已经借出可变引用后，后续不能再借出可变引用和不可变引用

- 写入向量元素

- 向量幕后的工作流程

**代码区**

```rust
#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: Vec::new(), // 另一种写法 vec![]
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File { name })
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    // 1. 创建向量
    let _array_1: [&str; 3] = ["sad", "happy", "gray"]; // 传统数组是一种固定大小、固定类型的数据结构

    let array_2: Vec<i32> = Vec::new(); // 方式一
    // let array_2 = Vec::<i32>::new(); // 方式二
    println!("{:?}", &array_2);

    let array_3: Vec<&str> = Vec::new();
    println!("{:?}", &array_3);

    let array_4 = vec![1, 2, 3, 4, 5]; // 该方式支持类型判断
    println!("{:?}", &array_4);

    // 2. 增加元素、删除元素
    let mut array_5 = vec![21.1, 3.2, 5.0, 6.2, 12.3];
    println!("Now, {:?}", &array_5);

    array_5.push(7.4);
    println!("After push, {:?}", &array_5);

    array_5.insert(2, 7.4);
    println!("After insert 2, {:?}", &array_5);

    array_5.insert(0, 7.4);
    println!("After insert 0, {:?}", &array_5);

    let option_1 = array_5.pop(); // pop 后得到一个可选枚举值
    println!("After pop, {:?}", &array_5);
    println!("option_1: {:?}", &option_1.unwrap_or(0.0));

    array_5.remove(0);
    println!("After remove 0, {:?}", &array_5);

    array_5.remove(5);
    println!("After remove 5, {:?}", &array_5);

    // 3. 读取元素
    let array_6 = vec!["qee", "gdr", "wrr", "ddz", "qwh"];
    println!("Now: {:?}", &array_6);

    println!("Get data 0 with immediately: {:?}", &array_6[0]); // 直接获得值，本处取的是字符引用
    println!("Get data 3 with immediately: {:?}", &array_6[3]);
    println!("Get data 4 with immediately: {:?}", &array_6[4]);

    println!("Get data with area [0..2]: {:?}", &array_6[0..2]); // 获得子数组
    println!("Get data with area [2..4]: {:?}", &array_6[2..5]);

    println!("Get data 0 with function: {:?}", array_6.get(0)); // get 后获得一个可选枚举值
    println!("Get data 3 with function: {:?}", array_6.get(3));
    println!(
        "Get data 4 with function: {:?}",
        array_6.get(4).unwrap_or(&"error")
    ); // 包含错误处理的解构处理

    // 4. get方法读取元素
    // 避免了索引不存在的问题

    match array_6.get(2) {
        None => println!("None is happen."),
        Some(value) => println!("Get data 3 with function: {:?}", value),
    }

    match array_6.get(7) {
        None => println!("None is happen."),
        Some(value) => println!("Get data 3 with function: {:?}", value),
    }

    // 5. 向量的所有权
    /*
        可以存在多个不可变引用和一个可变引用
        在创建一个可变引用后，原变量无法执行 push 等修改向量内部的操作
    */
    let array_7 = vec![
        String::from("djuue388"),
        String::from("ogipp842"),
        String::from("pohlk112"),
    ];
    let reference_1 = &array_7[0];
    let reference_2 = &array_7[0];

    println!("reference_1: {:?}", &reference_1);
    println!("reference_2: {:?}", &reference_2);

    // 6. 写入向量元素
    let mut array_8 = vec![
        String::from("djuue388"),
        String::from("ogipp842"),
        String::from("pohlk112"),
    ];

    // let copy_1 = array_8;

    array_8.push(String::from("ghurh664"));

    let reference_3 = &mut array_8[0];
    reference_3.push_str(" and hjjui998");

    println!("reference_3: {:?}", &reference_3);

    println!("Final Array is: {:?}", array_8);

    // 7. 向量幕后的工作流程
    let mut seasons = Vec::<&str>::with_capacity(4);

    println!("seasons length is: {}", seasons.len());
    println!("seasons capacity is: {}", seasons.capacity());
    println!("seasons data is: {:?}", seasons);

    seasons.push("Spring");
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");

    println!("seasons length is: {}", seasons.len());
    println!("seasons capacity is: {}", seasons.capacity());
    println!("seasons data is: {:?}", seasons);

    seasons.push("Other"); // 向量支持容量扩展

    println!("seasons length is: {}", seasons.len());
    println!("seasons capacity is: {}", seasons.capacity());
    println!("seasons data is: {:?}", seasons);

    // 8. test
    let mut folder = Folder::new("bestSelf".to_string());

    folder.create_file(String::from("popMusic1"));
    folder.create_file(String::from("countrysideMusic1"));

    println!("{:?}", folder);

    let delete_result_1 = folder.delete_file(0);

    println!("{:?}", folder);
    println!("delete_result_1 : {:?}", delete_result_1);

    match folder.get_file(0) {
        None => println!("There was no file"),
        Some(file) => println!("{file:?}"),
    }

    match folder.get_file(1) {
        None => println!("There was no file"),
        Some(file) => println!("{file:?}"),
    }
}
```

---

#### 项目结构

- 创建模块

- 声明模块属性的使用性

- 创建多个模块

- 文件模块

- 文件夹模块

- 模块模糊性

- 公共枚举、数据结构、领域

- 子级模块
  
  - 所有的模块默认都是私有的

- 项目前缀
  
  - 相对路径寻找模块`模块名称`
  
  - 绝对路径寻找模块`crate`

- 使用关键字一

- 使用关键字二

- 自我关键字

- 顶层关键字

- 给关键字设置别名

- 使用模块中的具体内容

- 外部依赖库

- 标准库

- 全局操作符

- 创建依赖包文件

- 创建多个依赖包文件

- 文档注释
  
  - 指令：`cargo doc --no-deps`，生成文档文件，但不存在于目前的项目中
  
  - 指令：`cargo doc --no-deps --open`，生成文档文件，但不存在于目前的项目中，自动在浏览器中打开

- 依赖包网站【`crates.io`】

- 测试

- 补充
  
  - `main.rs`是整个项目执行的入口文件，也可以在指令上自行指定
  
  - `lib.rs`是整个项目模块的入口文件，所有模块的使用注册都在这里执行

```rust
// main.rs
use project_14_test::GymWorkout;

fn main() {
    let data = GymWorkout::new();

    println!("The result is {:#?}", data);
} 

// lib.rs
mod cardio;

mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {}", NUTRITIONIST);
    }
}
mod weightlifting;

use cardio::{CardioTool, Exercise as CardioExercise};
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        cardio::ask_about_program();
        diet::ask_about_program();
        weightlifting::ask_about_program();

        return Self {
            cardio:{CardioExercise::new(String::from("20260413"),CardioTool::Bike,35)},
            weightlifting:{WeightliftingExercise::new(String::from("bibiLabu"),41)},
        };
    }
} 

// weightLifting.rs
const PERSONAL_TRAINER: &str = "Will Weight";

pub fn ask_about_program() {
    println!("The weightlifting trainer is {}", PERSONAL_TRAINER);
}

#[derive(Debug)]

pub struct Exercise {
    name: String,
    reps: u32,
}

impl Exercise {
    pub fn new(name: String, reps: u32) -> Self {
        return Self { name, reps };
    }
} 

// cardio/mod.rs
const PERSONAL_TRAINER: &str = "Carl Cardio";

pub fn ask_about_program() {
    println!("The cardio trainer is {}", PERSONAL_TRAINER);
}

#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Bike,
}

#[derive(Debug)]
pub struct Exercise {
    day: String,
    tool: CardioTool,
    minutes: u32,
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        return Self { day, tool, minutes };
    }
}
```

> 其余代码查看文档进行获取

---

#### 字符串(String)

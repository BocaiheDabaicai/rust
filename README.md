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

#### 代码区

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

#### 代码区

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

- 返回值，

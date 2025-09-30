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
